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

        let mut output_file = File::create(output_path)?;
        for palette in &tiled_indexed_image.palettes {
            let mut output_palette: [u16; 16] = [0xffff; 16];
            for (color_index, color) in palette.iter().enumerate() {
                // Color index 0 is reserved for the background, so add 1
                output_palette[color_index + 1] = color.packed();
            }

            let bytes: Vec<u8> = output_palette
                .iter()
                .copied()
                .flat_map(|packed| [packed as u8, (packed >> 8) as u8])
                .collect();
            output_file.write_all(&bytes)?;
        }
    }

    Ok(())
}
