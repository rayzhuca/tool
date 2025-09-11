use clap::Args;

#[derive(Args, Debug)]
pub struct SnippetArgs {
    name: Option<String>,
}

pub fn snippet(args: &SnippetArgs) {
    println!("snippet tool ran! {:?}", args);
}
