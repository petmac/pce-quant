mod true_color;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use true_color::TrueColorImage;

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Input: {}", cli.input_path.display());

    let input_image = TrueColorImage::decode(&cli.input_path)?;

    Ok(())
}
