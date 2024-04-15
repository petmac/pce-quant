use crate::{
    color::Color,
    tiled_image::{Tile, TILE_SIZE},
    tiled_indexed_image::IndexedPattern,
};

pub fn remap_tile(ideal_tile: &Tile, palette: &[Color]) -> IndexedPattern {
    let mut pattern = IndexedPattern::default();

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            pattern[y][x] = nearest_color_in_palette(&ideal_tile[y][x], palette) as u8;
        }
    }

    pattern
}

fn nearest_color_in_palette(ideal_color: &Color, palette: &[Color]) -> usize {
    let mut lowest_distance = manhattan_distance(ideal_color, &palette[0]);
    let mut nearest_color = 0;

    for color_index in 1..palette.len() {
        let distance = manhattan_distance(ideal_color, &palette[color_index]);
        if distance < lowest_distance {
            lowest_distance = distance;
            nearest_color = color_index;
        }
    }

    nearest_color
}

fn manhattan_distance(ideal_color: &Color, palette_color: &Color) -> f64 {
    (ideal_color.r - palette_color.r).abs()
        + (ideal_color.g - palette_color.g).abs()
        + (ideal_color.b - palette_color.b).abs()
}
