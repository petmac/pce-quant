use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{
    tiled_image::TILE_SIZE,
    tiled_indexed_image::{IndexedPattern, TiledIndexedImage},
};

pub struct Vram {
    pub batm: Vec<u8>,
    pub chr: Vec<Pattern>,
}

impl Vram {
    pub fn encode(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        w.write_all(&self.batm)?;

        for pattern in &self.chr {
            w.write_all(&pattern.bytes)?;
        }

        Ok(())
    }
}

impl From<&TiledIndexedImage> for Vram {
    fn from(image: &TiledIndexedImage) -> Self {
        let bat_width = ((image.width_in_tiles + 31) / 32) * 32;
        let bat_height = ((image.height_in_tiles + 31) / 32) * 32;
        let pattern_base_index = ((bat_width * bat_height) * 2) / 32;

        let mut tiles = image.tiles.iter().enumerate().map(|(tile_index, tile)| {
            (pattern_base_index + tile_index, tile.palette_index as usize)
        });

        let mut batm = Vec::new();
        for _bat_y in 0..image.height_in_tiles {
            for _bat_x in 0..image.width_in_tiles {
                let (pattern_index, palette_index) = tiles.next().unwrap_or_default();
                let pattern_index_word = pattern_index as u16;
                let palette_index_word = (palette_index << 12) as u16;
                let batm_word = pattern_index_word | palette_index_word;

                batm.push((batm_word & 0xff) as u8);
                batm.push(((batm_word >> 8) & 0xff) as u8);
            }
            for _bat_x in image.width_in_tiles..bat_width {
                batm.push(0);
                batm.push(0);
            }
        }
        for _bat_y in image.height_in_tiles..bat_height {
            for _bat_x in 0..bat_width {
                batm.push(0);
                batm.push(0);
            }
        }

        let chr = image
            .tiles
            .iter()
            .map(|tile| Pattern::from(&tile.pattern))
            .collect();

        Vram { batm, chr }
    }
}

pub struct Pattern {
    pub bytes: [u8; 32],
}

impl From<&IndexedPattern> for Pattern {
    fn from(input: &IndexedPattern) -> Self {
        let mut output = Pattern { bytes: [0; 32] };

        for y in 0..TILE_SIZE {
            let input_row = &input[y];

            for x in 0..TILE_SIZE {
                let color_index = input_row[x];
                let input_bit_0 = color_index & 1;
                let input_bit_1 = (color_index >> 1) & 1;
                let input_bit_2 = (color_index >> 2) & 1;
                let input_bit_3 = (color_index >> 3) & 1;

                let output_byte_0 = input_bit_0 << x;
                let output_byte_1 = input_bit_1 << x;
                let output_byte_2 = input_bit_2 << x;
                let output_byte_3 = input_bit_3 << x;

                output.bytes[y * 2] |= output_byte_0;
                output.bytes[y * 2 + 1] |= output_byte_1;
                output.bytes[y * 2 + 16] |= output_byte_2;
                output.bytes[y * 2 + 17] |= output_byte_3;
            }
        }

        output
    }
}
