use crate::prompt;
use crate::templates;
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// Dossier dans lequel `project create` crée les projets.
    pub projects_dir: String,
    /// Template proposé par défaut.
    pub default_template: String,
    /// `git init` automatique après le template.
    pub default_git: bool,
}

impl Default for Config {
    fn default() -> Self {
        let projects_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Projects");

        Self {
            projects_dir: projects_dir.display().to_string(),
            default_template: "rust".to_string(),
            default_git: true,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("lecture de {}", path.display()))?;

        let config: Config =
            toml::from_str(&contents).with_context(|| format!("parsing de {}", path.display()))?;

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("création de {}", parent.display()))?;
        }

        let contents = toml::to_string_pretty(self).context("sérialisation de la configuration")?;

        std::fs::write(path, contents)
            .with_context(|| format!("écriture de {}", path.display()))?;

        Ok(())
    }
}

/// Dossier de configuration, par ordre de priorité :
/// `$SHELLA_CONFIG_DIR`, `$XDG_CONFIG_HOME/shella-rs`, `~/.config/shella-rs`.
pub fn config_dir() -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os("SHELLA_CONFIG_DIR") {
        return Ok(PathBuf::from(dir));
    }

    if let Some(dir) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(dir).join("shella-rs"));
    }

    let home = dirs::home_dir().context("impossible de trouver le dossier home")?;
    Ok(home.join(".config").join("shella-rs"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE))
}

/// Remplace un `~` initial par le dossier home.
pub fn expand_home(path: &str) -> PathBuf {
    if path == "~" {
        return dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    }

    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }

    PathBuf::from(path)
}

/// Charge la config, ou lance le wizard de premier lancement.
pub fn load_or_onboard() -> Result<Config> {
    let path = config_path()?;

    if path.exists() {
        return Config::load(&path);
    }

    // Sans terminal (pipe, CI), impossible de poser des questions : défauts.
    if !prompt::is_interactive() {
        let config = Config::default();
        config.save(&path)?;
        return Ok(config);
    }

    onboard(&path)
}

/// Relance le wizard même si une config existe (`shella-rs --setup`).
pub fn setup() -> Result<Config> {
    let path = config_path()?;
    onboard(&path)
}

fn onboard(path: &Path) -> Result<Config> {
    println!("Bienvenue dans shella-rs ! Première configuration.\n");

    // On extrait les scripts embarqués avant de proposer les templates.
    templates::ensure_extracted()?;

    let raw_dir = prompt::ask_validated("Dossier où créer tes projets :", validate_directory)?;
    let projects_dir = expand_home(&raw_dir).display().to_string();

    let template_names = templates::names()?;
    let default_template = if template_names.is_empty() {
        "rust".to_string()
    } else {
        let names: Vec<&str> = template_names.iter().map(|name| name.as_str()).collect();
        let index = prompt::ask_choice("Template par défaut :", &names, 0)?;
        names[index].to_string()
    };

    let default_git = prompt::ask_yes_no("Initialiser git par défaut ?", true)?;

    let config = Config {
        projects_dir,
        default_template,
        default_git,
    };

    config.save(path)?;
    println!("\nConfiguration enregistrée dans {}\n", path.display());

    Ok(config)
}

/// Vérifie (et crée au besoin) le dossier saisi pendant le wizard.
fn validate_directory(input: &str) -> Result<(), String> {
    if input.trim().is_empty() {
        return Err("Le chemin ne peut pas être vide.".to_string());
    }

    let path = expand_home(input);

    if path.is_file() {
        return Err(format!(
            "{} est un fichier, pas un dossier.",
            path.display()
        ));
    }

    std::fs::create_dir_all(&path)
        .map_err(|err| format!("Impossible de créer {} : {err}", path.display()))
}
