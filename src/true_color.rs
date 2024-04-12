use std::{error::Error, fs::File, path::PathBuf};

use png::{Decoder, Transformations};

use crate::color::ColorU8;

pub struct TrueColorImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<ColorU8>,
}

impl TrueColorImage {
    pub fn decode(path: &PathBuf) -> Result<TrueColorImage, Box<dyn Error>> {
        let mut decoder = Decoder::new(File::open(path)?);
        decoder.set_transformations(Transformations::EXPAND);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let pixels = buf.chunks_exact(3).map(ColorU8::new).collect();

        Ok(TrueColorImage {
            width: info.width as usize,
            height: info.height as usize,
            pixels,
        })
    }
}
