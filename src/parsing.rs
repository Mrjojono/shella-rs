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

        let mut token_started = false;
        let mut in_single_quote = false;
        let mut in_double_quote = false;

        let mut chars = self.input.trim_end().chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\'' if !in_double_quote => {
                    token_started = true;
                    in_single_quote = !in_single_quote;
                }

                '"' if !in_single_quote => {
                    token_started = true;
                    in_double_quote = !in_double_quote;
                }

                c if c.is_whitespace() && !in_single_quote && !in_double_quote => {
                    if token_started {
                        args.push(current.clone());
                        current.clear();
                        token_started = false;
                    }
                }

                '\\' if in_double_quote => {
                    token_started = true;

                    if let Some(next) = chars.next() {
                        match next {
                            '"' | '\\' | '$' | '`' => current.push(next),
                            _ => {
                                current.push('\\');
                                current.push(next);
                            }
                        }
                    }
                }

                '\\' if !in_single_quote && !in_double_quote => {
                    token_started = true;

                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }

                '\\' if in_single_quote => {
                    token_started = true;
                    current.push('\\');
                }

                _ => {
                    token_started = true;
                    current.push(c);
                }
            }
        }

        if token_started {
            args.push(current);
        }

        args
    }
}
