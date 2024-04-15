use crate::{
    color::ColorU8,
    tiled_image::{Tile, TILE_SIZE},
    tiled_indexed_image::IndexedPattern,
};

pub fn remap(pixels: &[ColorU8], palette: &[ColorU8]) -> Vec<u8> {
    pixels
        .iter()
        .map(|col| nearest_color_in_palette(col, palette) as u8)
        .collect()
}

pub fn remap_tile(ideal_tile: &Tile, palette: &[ColorU8]) -> IndexedPattern {
    let mut pattern = IndexedPattern::default();

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            pattern[y][x] = nearest_color_in_palette(&ideal_tile[y][x], palette) as u8;
        }
    }

    pattern
}

fn nearest_color_in_palette(ideal_color: &ColorU8, palette: &[ColorU8]) -> usize {
    let nearest = palette
        .iter()
        .enumerate()
        .min_by_key(|&(_index, palette_color)| manhattan_distance(ideal_color, palette_color));
    match nearest {
        Some((nearest_index, _nearest_col)) => nearest_index,
        None => 0,
    }
}

fn manhattan_distance(ideal_color: &ColorU8, palette_color: &ColorU8) -> usize {
    ideal_color.r.abs_diff(palette_color.r) as usize
        + ideal_color.g.abs_diff(palette_color.g) as usize
        + ideal_color.b.abs_diff(palette_color.b) as usize
}
