use std::path::PathBuf;
use std::fs;
use clap::Parser;

#[derive(Parser,Default,Debug)]
//#[command(author, version, about, long_about = None)]
//#[command(propagate_version = true)]
struct Cli {
    template: PathBuf,
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
}
