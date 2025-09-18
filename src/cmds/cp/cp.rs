use ansi_term::Colour::Red;
use clap::{Args, Parser, Subcommand};
use dotenvy;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

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
struct CfArgs {
    contest: String,
}

#[derive(Args, Debug)]
struct RunArgs {
    file: String,
}

pub fn cp(args: &CpArgs) {
    match &args.subcommand {
        CpCmds::Cf(args) => cf(args),
        CpCmds::Run(args) => run(args),
    }
}

fn cf(args: &CfArgs) {
    let mut path = PathBuf::from(dotenvy::var("CF_DIR").unwrap());
    if !path.exists() || !path.is_dir() {
        println!("{}", Red.paint("CF_DIR env variable is not a directory"));
        process::exit(1);
    }

    path.push(args.contest.to_string());
    if path.exists() && !path.is_dir() {
        println!(
            "{}",
            Red.paint(format!("{:?} exists but is not a directory.", path))
        );
    }
    if !path.exists() {
        cf_create(&path);
    }
    cf_open(&path);
}

fn cf_open(path: &Path) {
    Command::new("code").arg("-n").arg(path).output().unwrap();
}

fn cf_create(path: &Path) {
    fs::create_dir(path).unwrap();
    cf_open(path);
}

fn run(args: &RunArgs) {
    println!("{:?}", args);
}
