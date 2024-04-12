use std::cmp::max;

use crate::{
    color::{ColorF, ColorU8},
    distribution::Distribution,
    indexed::IndexedImage,
    true_color::TrueColorImage,
};

const MAX_COLORS: usize = 16;

struct Tree {
    leaves: Vec<Distribution>,
}

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let distribution = Distribution::new(&input_image.pixels);
    let tree = build_tree(distribution);
    let palette: Vec<ColorU8> = build_palette(tree);
    let pixels = remap(&input_image.pixels, &palette);

    IndexedImage {
        width: input_image.width,
        height: input_image.height,
        palette,
        pixels,
    }
}

fn build_tree(distribution: Distribution) -> Tree {
    let mut leaves = vec![distribution];

    loop {
        let leaf_with_most_pixels = leaves
            .iter_mut()
            .filter(|distribution| distribution.unique_color_count() > 1)
            .max_by_key(|distribution| distribution.pixel_count());
        match leaf_with_most_pixels {
            Some(distribution) => {
                let (greater_equal, less) = partition_colors(distribution);
                if greater_equal.is_empty() || less.is_empty() {
                    println!("Stopping because split failed");
                    break;
                }

                *distribution = greater_equal;
                leaves.push(less);

                if leaves.len() >= MAX_COLORS {
                    println!("Stopping because we've got enough leaves");
                    break;
                }
            }
            None => {
                println!("Stopping because there are no leaves which can be split");
                break;
            }
        }
    }

    Tree { leaves: leaves }
}

fn partition_colors(distribution: &Distribution) -> (Distribution, Distribution) {
    let max_r = distribution.unique_colors().map(red).max();
    let min_r = distribution.unique_colors().map(red).min();
    let max_g = distribution.unique_colors().map(green).max();
    let min_g = distribution.unique_colors().map(green).min();
    let max_b = distribution.unique_colors().map(blue).max();
    let min_b = distribution.unique_colors().map(blue).min();
    match (max_r, min_r, max_g, min_g, max_b, min_b) {
        (Some(max_r), Some(min_r), Some(max_g), Some(min_g), Some(max_b), Some(min_b)) => {
            let r_range = max_r - min_r;
            let g_range = max_g - min_g;
            let b_range = max_b - min_b;
            let max_range = max(r_range, max(g_range, b_range));
            let (extract_component, extract_component_f): (fn(&ColorU8) -> u8, fn(&ColorF) -> f64) =
                if max_range == r_range {
                    (red, red_f)
                } else if max_range == g_range {
                    (green, green_f)
                } else {
                    (blue, blue_f)
                };
            partition_colors_by(distribution, extract_component, extract_component_f)
        }
        (_, _, _, _, _, _) => (Distribution::new(&[]), Distribution::new(&[])),
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

fn red_f(color: &ColorF) -> f64 {
    color.r
}

fn green_f(color: &ColorF) -> f64 {
    color.g
}

fn blue_f(color: &ColorF) -> f64 {
    color.b
}

fn partition_colors_by<F, G>(
    distribution: &Distribution,
    extract_component: F,
    extract_component_f: G,
) -> (Distribution, Distribution)
where
    F: Fn(&ColorU8) -> u8,
    G: Fn(&ColorF) -> f64,
{
    let avg_component = extract_component_f(&distribution.average_color());
    distribution.partition(|color| extract_component(color) as f64 >= avg_component)
}

fn build_palette(tree: Tree) -> Vec<ColorU8> {
    tree.leaves
        .iter()
        .map(Distribution::average_color)
        .map(|color| color.into())
        .collect()
}

fn remap(pixels: &[ColorU8], palette: &[ColorU8]) -> Vec<u8> {
    pixels
        .iter()
        .map(|col| nearest_color_in_palette(col, palette))
        .collect()
}

fn nearest_color_in_palette(ideal_color: &ColorU8, palette: &[ColorU8]) -> u8 {
    let nearest = palette
        .iter()
        .enumerate()
        .min_by_key(|&(_index, palette_color)| color_diff(ideal_color, palette_color));
    match nearest {
        Some((nearest_index, _nearest_col)) => nearest_index as u8,
        None => 0,
    }
}

fn color_diff(ideal_color: &ColorU8, palette_color: &ColorU8) -> usize {
    ideal_color.r.abs_diff(palette_color.r) as usize
        + ideal_color.g.abs_diff(palette_color.g) as usize
        + ideal_color.b.abs_diff(palette_color.b) as usize
}
