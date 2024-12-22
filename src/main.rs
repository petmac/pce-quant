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

    #[arg(long = "png")]
    png_output_path: Option<PathBuf>,

    #[arg(long = "vram")]
    vram_output_path: Option<PathBuf>,

    #[arg(long = "palettes")]
    palettes_output_path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Input:  {}", cli.input_path.display());

    let input_image = Image::decode(&cli.input_path)?;
    let input_tiled_image = TiledImage::from(input_image);
    let tiled_indexed_image = TiledIndexedImage::from(input_tiled_image);

    if let Some(output_path) = cli.png_output_path {
        println!("Output: {}", output_path.display());

        let output_tiled_image = TiledImage::from(&tiled_indexed_image);
        let output_image = Image::from(output_tiled_image);
        output_image.encode(&output_path)?;
    }
    if let Some(output_path) = cli.vram_output_path {
        println!("Output: {}", output_path.display());

        let output_vram = Vram::from(&tiled_indexed_image);
        output_vram.encode(&output_path)?;
    }
    if let Some(output_path) = cli.palettes_output_path {
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

    Ok(())
}
