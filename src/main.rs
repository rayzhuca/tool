mod cmds;

use clap::{Parser, Subcommand};

use cmds::cp::cp::{cp, CpArgs};
use cmds::snippet::snippet::{snippet, SnippetArgs};

use dotenvy;

#[derive(Parser)]
#[command(version, about)]
struct Tool {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Snippet(SnippetArgs),
    Cp(CpArgs),
}

fn main() {
    dotenvy::dotenv().unwrap();

    let tool = Tool::parse();

    match &tool.command {
        Commands::Snippet(args) => snippet(args),
        Commands::Cp(args) => cp(args),
    }
}
