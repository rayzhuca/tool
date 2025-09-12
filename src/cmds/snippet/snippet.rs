use ansi_term::Colour::Red;
use ansi_term::Style;

use clap::{arg, Args};

use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, process};

#[derive(Args, Debug)]
pub struct SnippetArgs {
    name: Option<String>,

    #[arg(short, long, default_value_t = false)]
    create: bool,
}

pub fn snippet(args: &SnippetArgs) {
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

fn get_store_path() -> PathBuf {
    return PathBuf::from(&format!(
        "{}/src/cmds/snippet/snippet_store/",
        env!("CARGO_MANIFEST_DIR")
    ));
}

fn list() {
    let entries = fs::read_dir(get_store_path().as_path()).unwrap();
    let mut snippets = vec![];
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension() != Some(OsStr::new("txt")) {
            eprintln!(
                "{}",
                Red.paint(format!(
                    "Unexpected non-text file: {}",
                    path.to_str().unwrap()
                ))
            );
            continue;
        }
        let file_name = path.file_stem().unwrap();
        snippets.push(file_name.to_str().unwrap().to_string());
    }
    snippets.sort();

    if snippets.is_empty() {
        println!("{}", Style::new().bold().paint("No snippets found."));
    } else {
        println!("{}", Style::new().bold().paint("Snippets:"));
        println!("  - {}", snippets.join("\n  - "));
    }
}

fn find(name: &str) {
    let mut path = get_store_path().clone();
    path.push(format!("{}.txt", name));
    let contents = fs::read_to_string(path).unwrap();
    print!("{}", contents);
}

fn create(name: &str) {
    println!("create called {:?}", name);
}
