mod bsp;
mod color;
mod distribution;
mod image;
mod indexed_image;
mod palette;
mod quantize;
mod tiled_image;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use image::Image;
use tiled_image::TiledImage;

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,
    output_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Input:  {}", cli.input_path.display());
    println!("Output: {}", cli.output_path.display());

    let input_image = Image::decode(&cli.input_path)?;
    let tiled_image = TiledImage::from(input_image);
    let output_image = Image::from(tiled_image);
    output_image.encode(&cli.output_path)?;

    Ok(())
}
