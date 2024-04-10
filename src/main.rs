mod true_color;

use std::{error::Error, fs::File, path::PathBuf};

use clap::Parser;
use png::{Decoder, Transformations};
use true_color::{TrueColor, TrueColorImage};

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Input: {}", cli.input_path.display());

    let input_image = decode(&cli.input_path)?;

    Ok(())
}

fn decode(path: &PathBuf) -> Result<TrueColorImage, Box<dyn Error>> {
    let mut decoder = Decoder::new(File::open(path)?);
    decoder.set_transformations(Transformations::EXPAND);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let pixels = buf.chunks_exact(3).map(TrueColor::new).collect();

    Ok(TrueColorImage {
        width: info.width as usize,
        height: info.height as usize,
        pixels,
    })
}
