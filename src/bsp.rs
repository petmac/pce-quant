use std::cmp::max;

use crate::{
    color::{ColorF, ColorU8},
    distribution::Distribution,
};

const MAX_COLORS: usize = 16;

pub struct BspTree {
    pub leaves: Vec<Distribution>,
}

impl BspTree {
    pub fn new(distribution: Distribution) -> BspTree {
        let mut leaves = vec![distribution];

        loop {
            let leaf_with_most_pixels = leaves
                .iter_mut()
                .filter(|leaf| leaf.unique_color_count() > 1)
                .max_by_key(|leaf| leaf.pixel_count());
            match leaf_with_most_pixels {
                Some(leaf) => {
                    let (greater_equal, less) = partition_leaf(leaf);
                    if greater_equal.is_empty() || less.is_empty() {
                        println!("Stopping because split failed");
                        break;
                    }

                    *leaf = greater_equal;
                    leaves.push(less);

                    if leaves.len() >= MAX_COLORS {
                        break;
                    }
                }
                None => {
                    println!("Stopping because there are no leaves which can be split");
                    break;
                }
            }
        }

        BspTree { leaves: leaves }
    }
}

fn partition_leaf(leaf: &Distribution) -> (Distribution, Distribution) {
    let max_r = leaf.unique_colors().map(red).max();
    let min_r = leaf.unique_colors().map(red).min();
    let max_g = leaf.unique_colors().map(green).max();
    let min_g = leaf.unique_colors().map(green).min();
    let max_b = leaf.unique_colors().map(blue).max();
    let min_b = leaf.unique_colors().map(blue).min();
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
            partition_leaf_by(leaf, extract_component, extract_component_f)
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

fn partition_leaf_by<F, G>(
    leaf: &Distribution,
    extract_component: F,
    extract_component_f: G,
) -> (Distribution, Distribution)
where
    F: Fn(&ColorU8) -> u8,
    G: Fn(&ColorF) -> f64,
{
    let avg_component = extract_component_f(&leaf.average_color());
    leaf.partition(|color| extract_component(color) as f64 >= avg_component)
}
