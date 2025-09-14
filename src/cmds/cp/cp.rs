use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CpArgs {
    #[command(subcommand)]
    subcommand: CpCmds,
}

#[derive(Subcommand, Debug)]
enum CpCmds {
    Cf(CfArgs),
    Run(RunArgs),
}

#[derive(Args, Debug)]
struct CfArgs {}

#[derive(Args, Debug)]
struct RunArgs {}

pub fn cp(args: &CpArgs) {
    println!("{:?}", args);
}
