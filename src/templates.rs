use crate::config;
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Scripts embarqués dans le binaire à la compilation.
/// Clé = nom du template, valeur = contenu du script.
const BUILTINS: &[(&str, &str)] = &[
    ("rust", include_str!("../templates/rust.sh")),
    ("node", include_str!("../templates/node.sh")),
    ("vide", include_str!("../templates/vide.sh")),
];

pub struct Template {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

/// `~/.config/shella-rs/templates`
pub fn templates_dir() -> Result<PathBuf> {
    Ok(config::config_dir()?.join("templates"))
}

fn script_path(name: &str) -> Result<PathBuf> {
    Ok(templates_dir()?.join(format!("{name}.sh")))
}

/// Extrait les scripts embarqués s'ils n'existent pas déjà.
/// Un fichier existant n'est **jamais** écrasé : l'utilisateur peut le modifier.
pub fn ensure_extracted() -> Result<()> {
    let dir = templates_dir()?;
    std::fs::create_dir_all(&dir).with_context(|| format!("création de {}", dir.display()))?;

    for (name, contents) in BUILTINS {
        let path = dir.join(format!("{name}.sh"));

        if path.exists() {
            continue;
        }

        write_script(&path, contents)?;
    }

    Ok(())
}

/// Réécrit les scripts embarqués, en écrasant les modifications locales.
pub fn refresh() -> Result<()> {
    let dir = templates_dir()?;
    std::fs::create_dir_all(&dir).with_context(|| format!("création de {}", dir.display()))?;

    for (name, contents) in BUILTINS {
        write_script(&dir.join(format!("{name}.sh")), contents)?;
    }

    Ok(())
}

fn write_script(path: &Path, contents: &str) -> Result<()> {
    std::fs::write(path, contents).with_context(|| format!("écriture de {}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms)?;
    }

    Ok(())
}

/// Liste les templates disponibles (tous les `.sh` du dossier de config).
pub fn list() -> Result<Vec<Template>> {
    let dir = templates_dir()?;

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut templates = Vec::new();

    for entry in std::fs::read_dir(&dir).with_context(|| format!("lecture de {}", dir.display()))? {
        let path = entry?.path();

        if path.extension().and_then(|ext| ext.to_str()) != Some("sh") {
            continue;
        }

        let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };

        // La description vient de la ligne `# description: ...` du script.
        let description = std::fs::read_to_string(&path)
            .ok()
            .and_then(|contents| {
                contents
                    .lines()
                    .find_map(|line| line.strip_prefix("# description:"))
                    .map(|text| text.trim().to_string())
            })
            .unwrap_or_else(|| name.to_string());

        templates.push(Template {
            name: name.to_string(),
            description,
            path,
        });
    }

    templates.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(templates)
}

/// Noms des templates, triés.
pub fn names() -> Result<Vec<String>> {
    Ok(list()?.into_iter().map(|template| template.name).collect())
}

/// Exécute un template : `bash <script> <nom-du-projet> <dossier-cible>`.
///
/// Le nom et le dossier sont passés comme arguments séparés (pas de `sh -c`),
/// donc impossible de s'échapper via des caractères spéciaux.
pub fn run(template: &str, project_name: &str, target: &Path, use_git: bool) -> Result<()> {
    let script = script_path(template)?;

    if !script.exists() {
        let available = names().unwrap_or_default().join(", ");
        bail!(
            "template {template:?} introuvable ({}) — disponibles : {available}",
            script.display()
        );
    }

    let status = Command::new("bash")
        .arg(&script)
        .arg(project_name)
        .arg(target)
        .env("SHELLA_PROJECT_NAME", project_name)
        .env("SHELLA_TARGET_DIR", target)
        .env("SHELLA_GIT", if use_git { "1" } else { "0" })
        .status()
        .with_context(|| format!("exécution de {}", script.display()))?;

    if !status.success() {
        bail!(
            "le template {template:?} a échoué (code {:?})",
            status.code()
        );
    }

    Ok(())
}
