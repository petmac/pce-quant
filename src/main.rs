use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!("Input: {}", cli.input_path.display());
}
