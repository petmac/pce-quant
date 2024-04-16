use crate::{
    color::Color,
    tiled_image::{Tile, TILE_SIZE},
    tiled_indexed_image::IndexedPattern,
};

const ERROR_CORRECTION: f64 = 0.25;

pub fn remap_tile(ideal_tile: &Tile, palette: &[Color]) -> IndexedPattern {
    let mut pattern = IndexedPattern::default();

    for y in 0..TILE_SIZE {
        let mut error = [0.0; 3];

        for x in 0..TILE_SIZE {
            let ideal_color = &ideal_tile[y][x];
            let target_color = Color {
                r: ideal_color.r - error[0] * ERROR_CORRECTION,
                g: ideal_color.g - error[1] * ERROR_CORRECTION,
                b: ideal_color.b - error[2] * ERROR_CORRECTION,
            };
            let nearest_color_index = nearest_color_in_palette(&target_color, palette);
            let nearest_color = &palette[nearest_color_index];

            error[0] = nearest_color.r - target_color.r;
            error[1] = nearest_color.g - target_color.g;
            error[2] = nearest_color.b - target_color.b;

            pattern[y][x] = nearest_color_index as u8;
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
