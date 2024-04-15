use clustering::Elem;

use crate::color::ColorU8;

pub struct ColorDistribution {
    min: ColorU8,
    max: ColorU8,
}

impl ColorDistribution {
    pub fn new(pixels: &[ColorU8]) -> ColorDistribution {
        let min_r = pixels.iter().map(red).min().unwrap_or_default();
        let min_g = pixels.iter().map(green).min().unwrap_or_default();
        let min_b = pixels.iter().map(blue).min().unwrap_or_default();
        let max_r = pixels.iter().map(red).max().unwrap_or_default();
        let max_g = pixels.iter().map(green).max().unwrap_or_default();
        let max_b = pixels.iter().map(blue).max().unwrap_or_default();
        ColorDistribution {
            min: ColorU8 {
                r: min_r,
                g: min_g,
                b: min_b,
            },
            max: ColorU8 {
                r: max_r,
                g: max_g,
                b: max_b,
            },
        }
    }
}

impl Elem for ColorDistribution {
    fn dimensions(&self) -> usize {
        6
    }

    fn at(&self, i: usize) -> f64 {
        let component = match i {
            0 => self.min.r,
            1 => self.min.g,
            2 => self.min.b,
            3 => self.max.r,
            4 => self.max.g,
            _ => self.max.b,
        };
        component as f64
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
