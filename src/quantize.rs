use std::{cmp::max, collections::BTreeMap};

use crate::{color::Rgb8, indexed::IndexedImage, true_color::TrueColorImage};

const MAX_COLORS: usize = 16;

struct Tree {
    leaves: Vec<Histogram>,
}

type Histogram = BTreeMap<Rgb8, usize>;

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let histogram = build_histogram(&input_image.pixels);
    let tree = build_tree(histogram);
    let palette: Vec<Rgb8> = build_palette(tree);
    let pixels = remap(&input_image.pixels, &palette);

    IndexedImage {
        width: input_image.width,
        height: input_image.height,
        palette,
        pixels,
    }
}

fn build_histogram(pixels: &[Rgb8]) -> Histogram {
    let mut histogram = Histogram::new();

    for color in pixels {
        match histogram.get_mut(color) {
            Some(pixel_count) => *pixel_count += 1,
            None => {
                histogram.insert(*color, 1);
            }
        }
    }

    histogram
}

fn build_tree(histogram: Histogram) -> Tree {
    let mut leaves = vec![histogram];

    loop {
        let histogram_with_most_pixels = leaves
            .iter_mut()
            .max_by_key(|histogram| pixel_count(histogram));
        match histogram_with_most_pixels {
            Some(histogram) => {
                if histogram.len() <= 1 {
                    println!("Stopping because leaf only has 1 color");
                    break;
                }

                let (greater_equal, less) = partition_colors(histogram);
                if greater_equal.is_empty() || less.is_empty() {
                    println!("Stopping because split failed");
                    break;
                }

                *histogram = greater_equal;
                leaves.push(less);

                if leaves.len() >= MAX_COLORS {
                    println!("Stopping because we've got enough leaves");
                    break;
                }
            }
            None => {
                println!("Stopping because there are no leaves");
                break;
            }
        }
    }

    Tree { leaves: leaves }
}

fn pixel_count(histogram: &Histogram) -> usize {
    histogram.values().sum()
}

fn partition_colors(histogram: &Histogram) -> (Histogram, Histogram) {
    let max_r = histogram.keys().map(red).max();
    let min_r = histogram.keys().map(red).min();
    let max_g = histogram.keys().map(green).max();
    let min_g = histogram.keys().map(green).min();
    let max_b = histogram.keys().map(blue).max();
    let min_b = histogram.keys().map(blue).min();
    match (max_r, min_r, max_g, min_g, max_b, min_b) {
        (Some(max_r), Some(min_r), Some(max_g), Some(min_g), Some(max_b), Some(min_b)) => {
            let r_range = max_r - min_r;
            let g_range = max_g - min_g;
            let b_range = max_b - min_b;
            let max_range = max(r_range, max(g_range, b_range));
            let extract_component = if max_range == r_range {
                red
            } else if max_range == g_range {
                green
            } else {
                blue
            };
            partition_colors_by(histogram, extract_component)
        }
        (_, _, _, _, _, _) => (Histogram::new(), Histogram::new()),
    }
}

fn red(color: &Rgb8) -> u8 {
    color.r
}

fn green(color: &Rgb8) -> u8 {
    color.g
}

fn blue(color: &Rgb8) -> u8 {
    color.b
}

fn partition_colors_by<F>(histogram: &Histogram, extract_component: F) -> (Histogram, Histogram)
where
    F: Fn(&Rgb8) -> u8,
{
    let avg_color = average_color(histogram);
    let avg_component = extract_component(&avg_color);
    histogram
        .iter()
        .partition(|&(color, _count)| extract_component(color) >= avg_component)
}

fn average_color(histogram: &Histogram) -> Rgb8 {
    debug_assert!(!histogram.is_empty());

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut total_pixel_count = 0;

    for (color, &pixel_count) in histogram {
        r += color.r as usize * pixel_count;
        g += color.g as usize * pixel_count;
        b += color.b as usize * pixel_count;
        total_pixel_count += pixel_count;
    }

    Rgb8 {
        r: (r / total_pixel_count) as u8,
        g: (g / total_pixel_count) as u8,
        b: (b / total_pixel_count) as u8,
    }
}

fn build_palette(tree: Tree) -> Vec<Rgb8> {
    tree.leaves.iter().map(average_color).collect()
}

fn remap(pixels: &[Rgb8], palette: &[Rgb8]) -> Vec<u8> {
    pixels
        .iter()
        .map(|col| nearest_color_in_palette(col, palette))
        .collect()
}

fn nearest_color_in_palette(ideal_color: &Rgb8, palette: &[Rgb8]) -> u8 {
    let nearest = palette
        .iter()
        .enumerate()
        .min_by_key(|&(_index, palette_color)| color_diff(ideal_color, palette_color));
    match nearest {
        Some((nearest_index, _nearest_col)) => nearest_index as u8,
        None => 0,
    }
}

fn color_diff(ideal_color: &Rgb8, palette_color: &Rgb8) -> usize {
    ideal_color.r.abs_diff(palette_color.r) as usize
        + ideal_color.g.abs_diff(palette_color.g) as usize
        + ideal_color.b.abs_diff(palette_color.b) as usize
}
