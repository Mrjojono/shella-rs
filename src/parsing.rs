//recueille les arguments d'une commande en respectant les règles de parsing des quotes et des échappements
pub struct ParserInput {
    pub input: String,
}

// Parse the input string into a vector of arguments, respecting quotes and escape sequences.
impl ParserInput {
    pub fn new(input: String) -> Self {
        ParserInput { input }
    }

    // Parse the input string into a vector of arguments, respecting quotes and escape sequences.
    pub fn parse(&self) -> Vec<String> {
        let mut args = Vec::new();
        let mut current = String::new();

        let mut in_single_quote = false;
        let mut in_double_quote = false;

        //let chars = self.input.trim_end().chars();
        let mut chars = self.input.trim_end().chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\'' if !in_double_quote => {
                    //ceci veut dire que le prochain caractère doit être pris tel quel, sans être interprété comme une séquence d'échappement
                    in_single_quote = !in_single_quote;
                }

                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                }

                ' ' if !in_single_quote && !in_double_quote => {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                }

                '\\' if in_double_quote => {
                    if let Some(next) = chars.next() {
                        match next {
                            '"' | '\\' | '$' | '`' => current.push(next),

                            _ => {
                                current.push('\\');
                                current.push(next)
                            }
                        }
                    }
                }
                '\\' if !in_single_quote && !in_double_quote => {
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }

                '\\' if in_single_quote => {
                    current.push('\\');
                }
                _ => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            args.push(current);
        }

        args
    }
}
