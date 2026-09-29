use crate::args::parse_args;

pub fn handle_project(args: &[String]) {
    if args.is_empty() {
        println!("project: missing subcommand");
        return;
    }

    match args[0].as_str() {
        "create" => handle_create(&args[1..]),

        "list" => {
            println!("Listing projects...");
        }

        _ => {
            println!("project: unknown subcommand");
        }
    }
}

fn handle_create(args: &[String]) {
    let parsed = parse_args(args);

    if parsed.positional.is_empty() {
        println!("project create: missing project name");
        return;
    }

    let project_name = &parsed.positional[0];

    let template = parsed.get_option("template");
    let use_git = parsed.has_flag("git");
    let verbose = parsed.has_flag("verbose");

    println!("Project: {}", project_name);
    println!("Template: {:?}", template);
    println!("Git: {}", use_git);
    println!("Verbose: {}", verbose);
}
