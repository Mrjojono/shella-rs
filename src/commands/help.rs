use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn show_help() -> io::Result<()> {
    let help_file_path = File::open("../utils/help.txt");

    let fichier = match help_file_path {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open help file: {}", e);
            return Err(e);
        }
    };

    let lecteur = BufReader::new(fichier);

    for ligne in lecteur.lines() {
        println!("{}", ligne?);
    }

    Ok(())
}

pub fn handle_help(args: &[String]) {
    if args.is_empty() {
        println!("Please specify the type of help you want.");
        return;
    }

    match args[0].as_str() {
        "list" => {
            if let Err(e) = show_help() {
                eprintln!("Error reading help file: {}", e);
            }
        }
        _ => {
            println!("help: unknown subcommand");
        }
    }
}
