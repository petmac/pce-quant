use std::collections::BTreeMap;

use crate::color::{ColorF, ColorU8};

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

    pub fn is_empty(&self) -> bool {
        self.color_pixel_counts.is_empty()
    }

    pub fn unique_colors(&self) -> impl Iterator<Item = &ColorU8> {
        self.color_pixel_counts.keys()
    }

    pub fn unique_color_count(&self) -> usize {
        self.color_pixel_counts.len()
    }

    pub fn pixel_count(&self) -> usize {
        self.color_pixel_counts.values().sum()
    }

    pub fn average_color(&self) -> ColorF {
        debug_assert!(!self.color_pixel_counts.is_empty());

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut total_pixel_count = 0;

        for (color, pixel_count) in &self.color_pixel_counts {
            r += color.r as usize * pixel_count;
            g += color.g as usize * pixel_count;
            b += color.b as usize * pixel_count;
            total_pixel_count += pixel_count;
        }

        let scale = 1.0 / total_pixel_count as f64;
        ColorF {
            r: r as f64 * scale,
            g: g as f64 * scale,
            b: b as f64 * scale,
        }
    }

    pub fn partition<F>(&self, mut f: F) -> (ColorDistribution, ColorDistribution)
    where
        F: FnMut(&ColorU8) -> bool,
    {
        let (f_true, f_false) = self
            .color_pixel_counts
            .iter()
            .partition(|&(color, _count)| f(color));
        (
            ColorDistribution {
                color_pixel_counts: f_true,
            },
            ColorDistribution {
                color_pixel_counts: f_false,
            },
        )
    }
}
