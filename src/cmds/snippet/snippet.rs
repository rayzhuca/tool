use clap::{arg, Args};

use std::process;

#[derive(Args, Debug)]
pub struct SnippetArgs {
    name: Option<String>,

    #[arg(short, long, default_value_t = true)]
    create: bool,
}

pub fn snippet(args: &SnippetArgs) {
    println!("snippet tool ran! {:?}", args);
    if let Some(name) = &args.name {
        if args.create {
            create(&name);
        } else {
            find(&name);
        }
    } else {
        if args.create {
            eprintln!("Snippet name must be provided.");
            process::exit(1);
        }
        list();
    }
}

fn list() {
    println!("list called");
}

fn find(name: &str) {
    println!("find called {:?}", name);
}

fn create(name: &str) {
    println!("create called {:?}", name);
}
