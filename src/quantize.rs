use crate::{
    bsp::BspTree,
    color::{ColorU3, ColorU8},
    distribution::Distribution,
    indexed::IndexedImage,
    true_color::TrueColorImage,
};

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let distribution = Distribution::new(&input_image.pixels);
    let tree = BspTree::new(distribution);
    let palette: Vec<ColorU3> = build_palette(tree);
    let pixels = remap(&input_image.pixels, &palette);

    IndexedImage {
        width: input_image.width,
        height: input_image.height,
        palette,
        pixels,
    }
}

fn build_palette(tree: BspTree) -> Vec<ColorU3> {
    tree.leaves
        .iter()
        .map(Distribution::average_color)
        .map(|color| color.into())
        .collect()
}

fn remap(pixels: &[ColorU8], palette: &[ColorU3]) -> Vec<u8> {
    let u8_palette: Vec<ColorU8> = palette.iter().copied().map(ColorU3::into).collect();

    pixels
        .iter()
        .map(|col| nearest_color_in_palette(col, &u8_palette))
        .collect()
}

fn nearest_color_in_palette(ideal_color: &ColorU8, palette: &[ColorU8]) -> u8 {
    let nearest = palette
        .iter()
        .enumerate()
        .min_by_key(|&(_index, palette_color)| manhattan_distance(ideal_color, palette_color));
    match nearest {
        Some((nearest_index, _nearest_col)) => nearest_index as u8,
        None => 0,
    }
}

fn manhattan_distance(ideal_color: &ColorU8, palette_color: &ColorU8) -> usize {
    ideal_color.r.abs_diff(palette_color.r) as usize
        + ideal_color.g.abs_diff(palette_color.g) as usize
        + ideal_color.b.abs_diff(palette_color.b) as usize
}
