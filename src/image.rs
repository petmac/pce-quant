use std::{error::Error, fs::File, io::BufWriter, path::PathBuf};

use png::{BitDepth, ColorType, Decoder, Encoder, Transformations};

use crate::color::Color;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn decode(path: &PathBuf) -> Result<Image, Box<dyn Error>> {
        let mut decoder = Decoder::new(File::open(path)?);
        decoder.set_transformations(Transformations::EXPAND);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let pixels = buf
            .chunks_exact(3)
            .map(|bytes| Color {
                r: bytes[0] as f64 / 255.0,
                g: bytes[1] as f64 / 255.0,
                b: bytes[2] as f64 / 255.0,
            })
            .collect();

        Ok(Image {
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
            .flat_map(|color| {
                [
                    (color.r * 255.0).round() as u8,
                    (color.g * 255.0).round() as u8,
                    (color.b * 255.0).round() as u8,
                ]
            })
            .collect();
        writer.write_image_data(&data)?;

        Ok(())
    }
}
