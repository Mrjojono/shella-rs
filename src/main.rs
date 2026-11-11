mod args;
mod cmd;
mod commands;
mod config;
mod parsing;
mod prompt;
mod templates;

use cmd::ShellCommand;
use parsing::ParserInput;
use std::io::{self, Write};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    // Options d'administration, traitées avant le lancement du shell.
    let mut argv = std::env::args().skip(1);

    if let Some(arg) = argv.next() {
        match arg.as_str() {
            "--setup" => {
                let config = config::setup()?;
                println!(
                    "Config : template={:?}, git={}, projets={}",
                    config.default_template, config.default_git, config.projects_dir
                );
                return Ok(());
            }
            "--refresh-templates" => {
                templates::refresh()?;
                println!(
                    "Templates restaurés dans {}",
                    templates::templates_dir()?.display()
                );
                return Ok(());
            }
            "--version" | "-V" => {
                println!("shella-rs {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "-h" => {
                println!("shella-rs — shell avec bootstrap de projets\n");
                println!("Usage : shella-rs [--setup | --refresh-templates | --version]");
                return Ok(());
            }
            other => {
                eprintln!("argument inconnu : {other}");
                std::process::exit(2);
            }
        }
    }

    let config = config::load_or_onboard()?;
    templates::ensure_extracted()?;

    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut input = String::new();

        // Ok(0) = EOF (Ctrl-D) : on quitte proprement.
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        let parts = ParserInput::new(input).parse();

        if parts.is_empty() {
            continue;
        }

        if parts[0] == "exit" {
            break;
        }

        let new_cmd = ShellCommand::new(&parts[0], &parts[1..], &config);
        new_cmd.eval_command().unwrap_or_else(|err| {
            eprintln!("Error executing command: {err:#}");
        });
    }

    Ok(())
}
