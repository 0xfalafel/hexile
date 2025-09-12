use std::path::PathBuf;
use clap::Parser;

mod interpreter;
use interpreter::binary_template;

#[derive(Parser,Default,Debug)]
//#[command(author, version, about, long_about = None)]
//#[command(propagate_version = true)]
struct Cli {
    template: PathBuf,
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    binary_template(cli.template, cli.file);

    println!("Hello, world!");
}
