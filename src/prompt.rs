use std::io::{self, IsTerminal, Write};

/// `true` si stdin est un terminal : on peut poser des questions.
/// En mode redirigé (`echo ... | shella-rs`, CI), on ne doit rien demander.
pub fn is_interactive() -> bool {
    io::stdin().is_terminal()
}

/// Pose une question et renvoie la réponse, espaces de début/fin retirés.
pub fn ask(question: &str) -> io::Result<String> {
    print!("{} ", question);
    io::stdout().flush()?;

    let mut input = String::new();

    // read_line renvoie Ok(0) sur EOF : sans ça, on boucherait indéfiniment.
    if io::stdin().read_line(&mut input)? == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "entrée fermée (EOF) — question sans réponse",
        ));
    }

    Ok(input.trim().to_string())
}

/// Pose une question jusqu'à obtenir une réponse valide.
/// `validate` renvoie `Err(message)` pour redemander.
pub fn ask_validated(
    question: &str,
    validate: impl Fn(&str) -> Result<(), String>,
) -> io::Result<String> {
    loop {
        let answer = ask(question)?;

        match validate(&answer) {
            Ok(()) => return Ok(answer),
            Err(message) => println!("{}", message),
        }
    }
}

/// Comme `ask`, mais refuse une réponse vide.
pub fn ask_required(question: &str) -> io::Result<String> {
    ask_validated(question, |answer| {
        if answer.is_empty() {
            Err("Ce champ est obligatoire.".to_string())
        } else {
            Ok(())
        }
    })
}

/// Question oui/non. Entrée seule = valeur par défaut.
pub fn ask_yes_no(question: &str, default: bool) -> io::Result<bool> {
    let hint = if default { "[O/n]" } else { "[o/N]" };

    loop {
        let answer = ask(&format!("{} {}", question, hint))?.to_lowercase();

        match answer.as_str() {
            "" => return Ok(default),
            "o" | "oui" | "y" | "yes" => return Ok(true),
            "n" | "non" | "no" => return Ok(false),
            _ => println!("Réponds par oui (o) ou non (n)."),
        }
    }
}

/// Menu numéroté. Renvoie l'index choisi (0-based).
/// Entrée seule = `default`.
pub fn ask_choice(question: &str, options: &[&str], default: usize) -> io::Result<usize> {
    assert!(!options.is_empty(), "ask_choice: options vide");
    assert!(default < options.len(), "ask_choice: default hors bornes");

    println!("{}", question);

    for (i, option) in options.iter().enumerate() {
        let marker = if i == default { "*" } else { " " };
        println!("  {}) {} {}", i + 1, marker, option);
    }

    loop {
        let answer = ask(&format!("Choix [{}] :", default + 1))?;

        if answer.is_empty() {
            return Ok(default);
        }

        if let Ok(n) = answer.parse::<usize>() {
            if (1..=options.len()).contains(&n) {
                return Ok(n - 1);
            }
        }

        println!("Choix invalide (1-{}).", options.len());
    }
}
