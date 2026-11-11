use crate::commands::help::handle_help;
use crate::commands::project::handle_project;
use crate::config::Config;
use anyhow::Result;
use pathsearch::find_executable_in_path;
use std::process::Command;

pub struct ShellCommand<'a> {
    cmd: String,
    args: Vec<String>,
    config: &'a Config,
}

impl<'a> ShellCommand<'a> {
    pub fn new(cmd: &str, args: &[String], config: &'a Config) -> Self {
        Self {
            cmd: cmd.to_string(),
            args: args.to_vec(),
            config,
        }
    }

    pub fn eval_command(&self) -> Result<()> {
        let known_commands = ["exit", "echo", "type", "pwd", "cd", "project", "help"];

        if known_commands.contains(&self.cmd.as_str()) {
            match self.cmd.as_str() {
                "echo" => {
                    println!("{}", self.args.join(" "));
                }
                "type" => {
                    if self.args.is_empty() {
                        println!("type: missing argument");
                    } else {
                        let arg = &self.args[0];

                        if known_commands.contains(&arg.as_str()) {
                            println!("{} is a shell builtin", arg);
                        } else if let Some(path) = find_executable_in_path(&self.args[0]) {
                            println!("{} is {}", self.args[0], path.display());
                        } else {
                            println!("{}: not found", arg);
                        }
                    }
                }
                "pwd" => {
                    let path = std::env::current_dir()?;
                    println!("{}", path.display());
                }
                "cd" => {
                    let target = if self.args.is_empty() || self.args[0] == "~" {
                        std::env::var("HOME")?
                    } else if self.args[0].starts_with("~/") {
                        let home = std::env::var("HOME")?;
                        format!("{}/{}", home, &self.args[0][2..])
                    } else {
                        self.args[0].clone()
                    };

                    if std::env::set_current_dir(&target).is_err() {
                        println!("cd: {}: No such file or directory", target);
                    }
                }
                "project" => {
                    handle_project(&self.args, self.config)?;
                }
                "help" => {
                    handle_help(&self.args);
                }
                "exit" => {
                    return Ok(());
                }

                _ => {}
            }

            return Ok(());
        }

        // exécution de commande externe
        let result = Command::new(&self.cmd).args(&self.args).spawn();

        match result {
            Ok(mut child) => {
                child.wait()?;
            }

            Err(_) => {
                println!("{}: command not found", self.cmd);
            }
        }

        Ok(())
    }
}
