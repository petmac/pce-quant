use std::{error::Error, fs::File, io::BufWriter, path::PathBuf};

use png::{BitDepth, ColorType, Encoder};

use crate::color::ColorU8;

pub struct IndexedImage {
    pub width: usize,
    pub height: usize,
    pub palette: Vec<ColorU8>,
    pub pixels: Vec<u8>,
}

impl IndexedImage {
    pub fn encode(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let palette: Vec<u8> = self
            .palette
            .iter()
            .flat_map(|color| [color.r, color.g, color.b])
            .collect();

        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_palette(palette);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }
}
