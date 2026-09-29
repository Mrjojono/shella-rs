use std::collections::HashMap;

pub struct ParsedArgs {
    pub positional: Vec<String>,
    pub flags: Vec<String>,
    pub options: HashMap<String, String>,
}

pub fn parse_args(args: &[String]) -> ParsedArgs {
    let mut positional = Vec::new();
    let mut flags = Vec::new();
    let mut options = HashMap::new();

    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with("--") {
            let name = &arg[2..];

            if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                options.insert(name.to_string(), args[i + 1].clone());
                i += 2;
            } else {
                flags.push(name.to_string());
                i += 1;
            }
        } else {
            positional.push(arg.clone());
            i += 1;
        }
    }

    ParsedArgs {
        positional,
        flags,
        options,
    }
}

// Implement methods for ParsedArgs
impl ParsedArgs {
    //verify if a flag is present in the parsed arguments
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.iter().any(|flag| flag == name)
    }

    // Get the value of an option if it exists
    pub fn get_option(&self, name: &str) -> Option<&String> {
        self.options.get(name)
    }
}
