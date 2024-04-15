use std::iter::zip;

use clustering::kmeans;

use crate::{
    color::Color,
    color_distribution::ColorDistribution,
    palette::{Palette, MAX_PALETTE_COLORS},
    remap::remap_tile,
    tiled_image::{Tile, TiledImage, TILE_SIZE},
};

const MAX_PALETTES: usize = 16;

pub struct TiledIndexedImage {
    pub width_in_tiles: usize,
    pub height_in_tiles: usize,
    pub palettes: Vec<Palette>,
    pub tiles: Vec<IndexedTile>,
}

pub struct IndexedTile {
    pub palette_index: u8,
    pub pattern: IndexedPattern,
}

pub type IndexedPattern = [[u8; TILE_SIZE]; TILE_SIZE];

impl From<TiledImage> for TiledIndexedImage {
    fn from(source_image: TiledImage) -> TiledIndexedImage {
        let tile_palette_indices = tile_palette_indices(&source_image.tiles);
        let palette_tile_indices = palette_tile_indices(&tile_palette_indices);
        let palette_tiles: Vec<Vec<&Tile>> = palette_tile_indices
            .iter()
            .map(|tile_indices| {
                tile_indices
                    .iter()
                    .copied()
                    .map(|tile_index| &source_image.tiles[tile_index])
                    .collect()
            })
            .collect();
        let palettes: Vec<Palette> = palette_tiles
            .iter()
            .map(|tiles| tiles_palette(tiles))
            .collect();
        let tiles = zip(tile_palette_indices, &source_image.tiles)
            .map(|(palette_index, tile)| {
                let palette = &palettes[palette_index];
                let pattern = remap_tile(&tile, palette);
                IndexedTile {
                    palette_index: palette_index as u8,
                    pattern,
                }
            })
            .collect();
        TiledIndexedImage {
            width_in_tiles: source_image.width_in_tiles,
            height_in_tiles: source_image.height_in_tiles,
            palettes,
            tiles,
        }
    }
}

fn tile_palette_indices(tiles: &[Tile]) -> Vec<usize> {
    let tile_color_distributions: Vec<ColorDistribution> =
        tiles.iter().map(tile_color_distribution).collect();
    let tile_clustering = kmeans(MAX_PALETTES, &tile_color_distributions, 100);
    tile_clustering.membership
}

fn tile_color_distribution(tile: &Tile) -> ColorDistribution {
    let colors: Vec<Color> = tile.iter().flatten().copied().collect();
    ColorDistribution::new(&colors)
}

fn tiles_palette(tiles: &[&Tile]) -> Palette {
    let colors: Vec<Color> = tiles.iter().flat_map(|&tile| tile_colors(tile)).collect();
    let color_clustering = kmeans(MAX_PALETTE_COLORS, &colors, 100);
    let mut cluster_colors: Vec<Vec<Color>> = vec![Vec::new(); MAX_PALETTE_COLORS];

    for (element_index, &cluster_index) in color_clustering.membership.iter().enumerate() {
        cluster_colors[cluster_index].push(color_clustering.elements[element_index]);
    }

    let palette = cluster_colors
        .iter()
        .map(|colors| average_color(colors))
        .collect();

    palette
}

fn tile_colors(tile: &Tile) -> Vec<Color> {
    tile.iter().flatten().copied().collect()
}

fn average_color(colors: &[Color]) -> Color {
    let r = colors.iter().fold(0.0, |acc, color| acc + color.r);
    let g = colors.iter().fold(0.0, |acc, color| acc + color.g);
    let b = colors.iter().fold(0.0, |acc, color| acc + color.b);

    let scale = if colors.is_empty() {
        0.0
    } else {
        1.0 / colors.len() as f64
    };

    Color {
        r: r * scale,
        g: g * scale,
        b: b * scale,
    }
}

fn palette_tile_indices(tile_palette_indices: &[usize]) -> Vec<Vec<usize>> {
    let max_palette_index = tile_palette_indices.iter().copied().max();
    let palette_count = max_palette_index.unwrap_or(0) + 1;
    let mut palette_tile_indices = vec![Vec::new(); palette_count];

    for (tile_index, &tile_palette_index) in tile_palette_indices.iter().enumerate() {
        palette_tile_indices[tile_palette_index as usize].push(tile_index);
    }

    palette_tile_indices
}

impl From<TiledIndexedImage> for TiledImage {
    fn from(source_image: TiledIndexedImage) -> Self {
        let tiles = source_image
            .tiles
            .iter()
            .map(|tile| {
                tile_from_pattern_and_palette(
                    &tile.pattern,
                    &source_image.palettes[tile.palette_index as usize],
                )
            })
            .collect();
        TiledImage {
            width_in_tiles: source_image.width_in_tiles,
            height_in_tiles: source_image.height_in_tiles,
            tiles: tiles,
        }
    }
}

const DEBUG_TILE_PALETTES: bool = false;

fn tile_from_pattern_and_palette(pattern: &IndexedPattern, palette: &[Color]) -> Tile {
    let mut tile = [[Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    }; TILE_SIZE]; TILE_SIZE];

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let color_index = if DEBUG_TILE_PALETTES {
                ((y / 2) * 4) + (x / 2)
            } else {
                pattern[y][x] as usize
            };
            let color = palette[color_index];
            tile[y][x] = color;
        }
    }

    tile
}
