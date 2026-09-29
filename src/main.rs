mod args;
mod cmd;
mod commands;
mod parsing;
use cmd::ShellCommand;
use parsing::ParserInput;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let parts = ParserInput::new(input).parse();

        if parts.is_empty() {
            continue;
        }

        if parts[0] == "exit" {
            break;
        }

        let new_cmd = ShellCommand::new(&parts[0], &parts[1..]);
        new_cmd.eval_command().unwrap_or_else(|err| {
            eprintln!("Error executing command: {}", err);
        });
    }
}
