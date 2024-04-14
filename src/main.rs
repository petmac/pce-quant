mod bsp;
mod color;
mod distribution;
mod image;
mod indexed_image;
mod palette;
mod remap;
mod tiled_image;
mod tiled_indexed_image;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use image::Image;
use tiled_image::TiledImage;

use crate::tiled_indexed_image::TiledIndexedImage;

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
    let input_tiled_image = TiledImage::from(input_image);
    let tiled_indexed_image = TiledIndexedImage::from(input_tiled_image);
    let output_tiled_image = TiledImage::from(tiled_indexed_image);
    let output_image = Image::from(output_tiled_image);
    output_image.encode(&cli.output_path)?;

    Ok(())
}
