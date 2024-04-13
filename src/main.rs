mod bsp;
mod color;
mod distribution;
mod indexed;
mod quantize;
mod tiled;
mod true_color;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use tiled::TiledImage;
use true_color::TrueColorImage;

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,
    output_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Input:  {}", cli.input_path.display());
    println!("Output: {}", cli.output_path.display());

    let input_image = TrueColorImage::decode(&cli.input_path)?;
    let tiled_image = TiledImage::from(input_image);
    let output_image = TrueColorImage::from(tiled_image);
    output_image.encode(&cli.output_path)?;

    Ok(())
}
