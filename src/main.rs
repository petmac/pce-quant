mod color;
mod color_distribution;
mod image;
mod palette;
mod remap;
mod tiled_image;
mod tiled_indexed_image;

use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use image::Image;
use tiled_image::TiledImage;

use crate::tiled_indexed_image::TiledIndexedImage;

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Png { output_path: PathBuf },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Input:  {}", cli.input_path.display());

    let input_image = Image::decode(&cli.input_path)?;
    let input_tiled_image = TiledImage::from(input_image);
    let tiled_indexed_image = TiledIndexedImage::from(input_tiled_image);

    match cli.command {
        Commands::Png { output_path } => {
            println!("Output: {}", output_path.display());

            let output_tiled_image = TiledImage::from(tiled_indexed_image);
            let output_image = Image::from(output_tiled_image);
            output_image.encode(&output_path)?;
        }
    }

    Ok(())
}
