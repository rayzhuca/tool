mod cmds;

use clap::{Parser, Subcommand};

use cmds::snippet::{snippet, SnippetArgs};

#[derive(Parser)]
#[command(version, about)]
struct Tool {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Snippet(SnippetArgs),
}

fn main() {
    let tool = Tool::parse();

    match &tool.command {
        Commands::Snippet(args) => snippet(args),
    }
}
