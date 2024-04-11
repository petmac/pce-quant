use std::{error::Error, fs::File, io::BufWriter, path::PathBuf};

use png::{BitDepth, ColorType, Decoder, Encoder, Transformations};

use crate::color::Rgb8;

pub struct TrueColorImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Rgb8>,
}

impl TrueColorImage {
    pub fn decode(path: &PathBuf) -> Result<TrueColorImage, Box<dyn Error>> {
        let mut decoder = Decoder::new(File::open(path)?);
        decoder.set_transformations(Transformations::EXPAND);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let pixels = buf.chunks_exact(3).map(Rgb8::new).collect();

        Ok(TrueColorImage {
            width: info.width as usize,
            height: info.height as usize,
            pixels,
        })
    }

    pub fn encode(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|color| [color.r, color.g, color.b])
            .collect();
        writer.write_image_data(&data)?;

        Ok(())
    }
}
