use std::io;

fn show_help() -> io::Result<()> {
    // Contenu embarqué dans le binaire à la compilation.
    // Chemin relatif à ce fichier (src/commands/help.rs -> src/utils/help.txt)
    let contenu = include_str!("../utils/help.txt");

    for ligne in contenu.lines() {
        println!("{}", ligne);
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
