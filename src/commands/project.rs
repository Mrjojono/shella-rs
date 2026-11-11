use crate::args::parse_args;
use crate::config::{self, Config};
use crate::prompt;
use crate::templates;
use anyhow::{Context, Result, bail};
use std::process::Command;

/// Actions proposées par le menu quand on tape `project` tout seul.
const MENU: [&str; 3] = ["Créer un projet", "Lister les projets", "Annuler"];

/**
 * fonction qui gère la commande `project` et ses sous-commandes
 * (create, list, et un menu interactif si aucune sous-commande n'est fournie).
 */
pub fn handle_project(args: &[String], config: &Config) -> Result<()> {
    let Some(subcommand) = args.first() else {
        if !prompt::is_interactive() {
            bail!("project: sous-commande manquante (utilisation : project <create|list>)");
        }

        return match prompt::ask_choice("Que veux-tu faire ?", &MENU, 0)? {
            0 => handle_create(&[], config),
            1 => list_projects(config),
            _ => {
                println!("Annulé.");
                Ok(())
            }
        };
    };

    match subcommand.as_str() {
        "create" => handle_create(&args[1..], config),
        "list" => list_projects(config),
        other => bail!("project: sous-commande inconnue {other:?}"),
    }
}

fn handle_create(args: &[String], config: &Config) -> Result<()> {
    let parsed = parse_args(args);

    // Nom : argument positionnel, sinon question.
    let project_name = match parsed.positional.first() {
        Some(name) => name.clone(),
        None => {
            if !prompt::is_interactive() {
                bail!("project create: nom du projet manquant");
            }

            prompt::ask_validated("Nom du projet :", validate_project_name)?
        }
    };

    // Un nom passé en argument doit aussi être valide.
    if let Err(message) = validate_project_name(&project_name) {
        bail!("project create: {message}");
    }

    // Template : `--template`, sinon config, sinon menu dynamique.
    let template = match parsed.get_option("template") {
        Some(template) => template.clone(),
        None if !prompt::is_interactive() => config.default_template.clone(),
        None => {
            let available = templates::list()?;

            if available.is_empty() {
                bail!(
                    "project create: aucun template dans {}",
                    templates::templates_dir()?.display()
                );
            }

            let names: Vec<&str> = available.iter().map(|item| item.name.as_str()).collect();
            let default = names
                .iter()
                .position(|name| *name == config.default_template.as_str())
                .unwrap_or(0);

            let index = prompt::ask_choice("Template :", &names, default)?;
            names[index].to_string()
        }
    };

    // Git : `--git` / `--no-git`, sinon config, sinon question.
    let use_git = if parsed.has_flag("git") {
        true
    } else if parsed.has_flag("no-git") {
        false
    } else if prompt::is_interactive() {
        prompt::ask_yes_no("Initialiser un dépôt git ?", config.default_git)?
    } else {
        config.default_git
    };

    let projects_dir = config::expand_home(&config.projects_dir);
    let target = projects_dir.join(&project_name);

    if target.exists() {
        bail!("project create: {} existe déjà", target.display());
    }

    std::fs::create_dir_all(&target)
        .with_context(|| format!("création de {}", target.display()))?;

    println!("→ création de {project_name:?} avec le template {template:?}");

    if let Err(err) = templates::run(&template, &project_name, &target, use_git) {
        // Pas de dossier à moitié créé derrière nous.
        let _ = std::fs::remove_dir_all(&target);
        return Err(err);
    }

    // `git init` est fait par le CLI, pas par les scripts : c'est uniforme.
    if use_git {
        let status = Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(&target)
            .status()
            .with_context(|| format!("git init dans {}", target.display()))?;

        if !status.success() {
            bail!("git init a échoué dans {}", target.display());
        }
    }

    println!("Projet créé dans {}", target.display());
    Ok(())
}

fn list_projects(config: &Config) -> Result<()> {
    let projects_dir = config::expand_home(&config.projects_dir);

    if !projects_dir.exists() {
        println!(
            "Aucun projet : {} n'existe pas encore.",
            projects_dir.display()
        );
        return Ok(());
    }

    let mut names = Vec::new();

    for entry in std::fs::read_dir(&projects_dir)
        .with_context(|| format!("lecture de {}", projects_dir.display()))?
    {
        let path = entry?.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                names.push(name.to_string());
            }
        }
    }

    names.sort();

    if names.is_empty() {
        println!("Aucun projet dans {}.", projects_dir.display());
    } else {
        println!("Projets dans {} :", projects_dir.display());
        for name in names {
            println!("  {name}");
        }
    }

    Ok(())
}

/// Valide le nom d'un projet : non vide, sans séparateur de chemin, pas `.`/`..`.
fn validate_project_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Le nom du projet ne peut pas être vide.".to_string());
    }

    if name.contains('/') || name.contains('\\') {
        return Err("Le nom du projet ne peut pas contenir de séparateur de chemin.".to_string());
    }

    if name == "." || name == ".." {
        return Err("Le nom du projet ne peut pas être « . » ou « .. ».".to_string());
    }

    Ok(())
}
