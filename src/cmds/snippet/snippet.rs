use ansi_term::Colour::{Green, Red};
use ansi_term::Style;

use clap::{arg, Args};

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{fs, process};

use tempfile::NamedTempFile;

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
    let editor = env::var("EDITOR").unwrap_or("vim".to_string());
    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path();

    let status = Command::new(editor)
        .arg(&temp_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();

    if !status.success() {
        eprintln!("{}", Red.paint("Could not read user input."));
        process::exit(1);
    }

    let contents = fs::read_to_string(temp_path).unwrap();
    let message = contents.lines().collect::<Vec<_>>().join("\n").to_string();

    let mut snippet_path = get_store_path().clone();
    snippet_path.push(format!("{}.txt", name));

    fs::write(snippet_path, message).unwrap();
    println!("{}", Green.paint("Snippet saved!"));
}
