use std::{error::Error, fs::File, io::BufWriter, path::PathBuf};

use crate::tiled_indexed_image::TiledIndexedImage;

pub struct Vram {}

impl Vram {
    pub fn encode(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);
        todo!()
    }
}

impl From<&TiledIndexedImage> for Vram {
    fn from(image: &TiledIndexedImage) -> Self {
        todo!()
    }
}
