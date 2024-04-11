use std::collections::BTreeMap;

use crate::{color::Rgb8, indexed::IndexedImage, true_color::TrueColorImage};

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let color_counts = count_unique_colors(input_image);
    let avg_color = weighted_average_color(&color_counts);
    let palette: Vec<Rgb8> = input_image.pixels[0..16].iter().copied().collect();
    let pixels = vec![0; input_image.pixels.len()];

    IndexedImage {
        width: input_image.width,
        height: input_image.height,
        palette,
        pixels,
    }
}

fn count_unique_colors(image: &TrueColorImage) -> BTreeMap<Rgb8, usize> {
    let mut color_counts = BTreeMap::new();

    for color in &image.pixels {
        match color_counts.get_mut(color) {
            Some(existing) => *existing += 1,
            None => {
                color_counts.insert(*color, 1);
            }
        }
    }

    color_counts
}

fn weighted_average_color(color_counts: &BTreeMap<Rgb8, usize>) -> (f64, f64, f64) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut denominator = 0;

    for (color, &count) in color_counts {
        r += color.r as usize;
        g += color.g as usize;
        b += color.b as usize;
        denominator += count;
    }

    let scale = 1.0 / denominator as f64;
    (r as f64 * scale, g as f64 * scale, b as f64 * scale)
}
