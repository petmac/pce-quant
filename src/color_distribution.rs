use clustering::Elem;

use crate::color::Color;

pub struct ColorDistribution {
    min: Color,
    max: Color,
}

impl ColorDistribution {
    pub fn new(pixels: &[Color]) -> ColorDistribution {
        let mut min = pixels[0];
        let mut max = min;

        for color in pixels {
            if color.r < min.r {
                min.r = color.r;
            }
            if color.g < min.g {
                min.g = color.g;
            }
            if color.b < min.b {
                min.b = color.b;
            }
            if color.r > max.r {
                max.r = color.r;
            }
            if color.g > max.g {
                max.g = color.g;
            }
            if color.b > max.b {
                max.b = color.b;
            }
        }

        ColorDistribution { min, max }
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
