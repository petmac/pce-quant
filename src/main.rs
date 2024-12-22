mod color;
mod color_distribution;
mod image;
mod palette;
mod remap;
mod tiled_image;
mod tiled_indexed_image;
mod vram;

use std::{error::Error, fs::File, io::Write, path::PathBuf};

use clap::{Parser, Subcommand};
use color::Color;
use image::Image;
use tiled_image::TiledImage;

use crate::{tiled_indexed_image::TiledIndexedImage, vram::Vram};

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Png { output_path: PathBuf },
    Vram { output_path: PathBuf },
    Palettes { output_path: PathBuf },
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
        Commands::Vram { output_path } => {
            println!("Output: {}", output_path.display());

            let output_vram = Vram::from(&tiled_indexed_image);
            output_vram.encode(&output_path)?;
        }
        Commands::Palettes { output_path } => {
            println!("Output: {}", output_path.display());

            let output_palettes: Vec<u8> = tiled_indexed_image
                .palettes
                .iter()
                .flatten()
                .map(|color| {
                    println!("Color: {}, {}, {}", color.r, color.g, color.b);
                    color
                })
                .map(Color::packed)
                .flat_map(|packed| [packed as u8, (packed >> 8) as u8])
                .collect();
            File::create(output_path)?.write_all(&output_palettes)?;
        }
    }

    Ok(())
}
