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
    colors: BTreeMap<Rgb8, usize>,
}

enum Channel {
    Red,
    Green,
    Blue,
}

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let colors = count_unique_colors(&input_image.pixels);
    let tree = build_tree(&colors, Channel::Red);
    let palette: Vec<Rgb8> = build_palette(tree);
    let pixels = remap(&input_image.pixels, &palette);

    IndexedImage {
        width: input_image.width,
        height: input_image.height,
        palette,
        pixels,
    }
}

fn count_unique_colors(pixels: &[Rgb8]) -> BTreeMap<Rgb8, usize> {
    let mut unique_colors = BTreeMap::new();

    for input_color in pixels {
        match unique_colors.get_mut(input_color) {
            Some(count) => *count += 1,
            None => {
                unique_colors.insert(*input_color, 1);
            }
        }
    }

    unique_colors
}

fn build_tree(colors: &BTreeMap<Rgb8, usize>, cut_channel: Channel) -> Tree {
    match cut_channel {
        Channel::Red => {
            let (greater_equal, less) = partition_colors_by(colors, red);
            let node = Node {
                greater_equal: build_tree(&greater_equal, Channel::Green),
                less: build_tree(&less, Channel::Green),
            };
            Tree::Node(Box::new(node))
        }
        Channel::Green => {
            let (greater_equal, less) = partition_colors_by(colors, green);
            let node = Node {
                greater_equal: build_tree(&greater_equal, Channel::Blue),
                less: build_tree(&less, Channel::Blue),
            };
            Tree::Node(Box::new(node))
        }
        Channel::Blue => {
            let (greater_equal, less) = partition_colors_by(colors, blue);
            let node = Node {
                greater_equal: Tree::Leaf(Leaf {
                    colors: greater_equal,
                }),
                less: Tree::Leaf(Leaf { colors: less }),
            };
            Tree::Node(Box::new(node))
        }
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

fn partition_colors_by<F>(
    colors: &BTreeMap<Rgb8, usize>,
    extract_component: F,
) -> (BTreeMap<Rgb8, usize>, BTreeMap<Rgb8, usize>)
where
    F: Fn(&Rgb8) -> u8,
{
    let avg_color = average_color(colors);
    let avg_component = extract_component(&avg_color);
    colors
        .iter()
        .partition(|&(color, _count)| extract_component(color) >= avg_component)
}

fn average_color(colors: &BTreeMap<Rgb8, usize>) -> Rgb8 {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut denominator = 0;

    for (color, count) in colors {
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
        Tree::Leaf(leaf) => vec![average_color(&leaf.colors)],
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
