use std::collections::BTreeMap;

use crate::{color::Rgb8, indexed::IndexedImage, true_color::TrueColorImage};

enum Tree {
    Node(Box<Node>),
    Leaf(Leaf),
}

struct Node {
    greater_equal: Tree,
    less: Tree,
}

struct Leaf {
    color_counts: BTreeMap<Rgb8, usize>,
}

enum Channel {
    Red,
    Green,
    Blue,
}

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let color_counts = count_unique_colors(input_image);
    let tree = build_tree(color_counts, Channel::Red);
    let palette: Vec<Rgb8> = build_palette(tree);
    let pixels = remap(&input_image.pixels, &palette);

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

fn build_tree(color_counts: BTreeMap<Rgb8, usize>, cut_channel: Channel) -> Tree {
    let avg_color = weighted_average_color(&color_counts);
    match cut_channel {
        Channel::Red => {
            let (greater_equal, less): (BTreeMap<Rgb8, usize>, BTreeMap<Rgb8, usize>) =
                color_counts
                    .iter()
                    .partition(|&(col, _count)| col.r >= avg_color.r);
            let node = Node {
                greater_equal: build_tree(greater_equal, Channel::Green),
                less: build_tree(less, Channel::Green),
            };
            Tree::Node(Box::new(node))
        }
        Channel::Green => {
            let (greater_equal, less): (BTreeMap<Rgb8, usize>, BTreeMap<Rgb8, usize>) =
                color_counts
                    .iter()
                    .partition(|&(col, _count)| col.g >= avg_color.g);
            let node = Node {
                greater_equal: build_tree(greater_equal, Channel::Blue),
                less: build_tree(less, Channel::Blue),
            };
            Tree::Node(Box::new(node))
        }
        Channel::Blue => {
            let (greater_equal, less): (BTreeMap<Rgb8, usize>, BTreeMap<Rgb8, usize>) =
                color_counts
                    .iter()
                    .partition(|&(col, _count)| col.b >= avg_color.b);
            let node = Node {
                greater_equal: Tree::Leaf(Leaf {
                    color_counts: greater_equal,
                }),
                less: Tree::Leaf(Leaf { color_counts: less }),
            };
            Tree::Node(Box::new(node))
        }
    }
}

fn weighted_average_color(color_counts: &BTreeMap<Rgb8, usize>) -> Rgb8 {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut denominator = 0;

    for (color, &count) in color_counts {
        r += color.r as usize * count;
        g += color.g as usize * count;
        b += color.b as usize * count;
        denominator += count;
    }

    Rgb8 {
        r: (r / denominator) as u8,
        g: (g / denominator) as u8,
        b: (b / denominator) as u8,
    }
}

fn build_palette(tree: Tree) -> Vec<Rgb8> {
    match tree {
        Tree::Node(node) => {
            let mut greater_equal = build_palette(node.greater_equal);
            let mut less = build_palette(node.less);
            less.append(&mut greater_equal);
            less
        }
        Tree::Leaf(leaf) => vec![weighted_average_color(&leaf.color_counts)],
    }
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
