use std::{error::Error, fs::File, path::PathBuf};

use png::{Decoder, Transformations};

pub struct TrueColorImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<TrueColor>,
}

impl TrueColorImage {
    pub fn decode(path: &PathBuf) -> Result<TrueColorImage, Box<dyn Error>> {
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
}

pub struct TrueColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TrueColor {
    pub fn new(rgb: &[u8]) -> Self {
        TrueColor {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}
