use std::collections::BTreeMap;

use clustering::Elem;

use crate::color::ColorU8;

pub struct ColorDistribution {
    color_pixel_counts: BTreeMap<ColorU8, usize>,
}

impl ColorDistribution {
    pub fn new(pixels: &[ColorU8]) -> ColorDistribution {
        let mut color_pixel_counts = BTreeMap::new();

        for color in pixels {
            match color_pixel_counts.get_mut(color) {
                Some(pixel_count) => *pixel_count += 1,
                None => {
                    color_pixel_counts.insert(*color, 1);
                }
            }
        }

        ColorDistribution { color_pixel_counts }
    }
}

impl Elem for ColorDistribution {
    fn dimensions(&self) -> usize {
        3
    }

    fn at(&self, i: usize) -> f64 {
        let channel_selector = match i {
            0 => red,
            1 => green,
            _ => blue,
        };

        let pixel_counts_for_dimension: usize = self
            .color_pixel_counts
            .iter()
            .map(|(color, &pixel_count)| channel_selector(color) as usize * pixel_count)
            .sum();
        pixel_counts_for_dimension as f64
    }
}

fn red(color: &ColorU8) -> u8 {
    color.r
}

fn green(color: &ColorU8) -> u8 {
    color.g
}

fn blue(color: &ColorU8) -> u8 {
    color.b
}
