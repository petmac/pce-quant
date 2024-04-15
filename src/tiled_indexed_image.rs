use std::iter::zip;

use clustering::kmeans;

use crate::{
    color::{ColorU3, ColorU8},
    color_distribution::ColorDistribution,
    palette::{PaletteU3, PaletteU8, MAX_PALETTE_COLORS},
    remap::remap_tile,
    tiled_image::{Tile, TiledImage, TILE_SIZE},
};

const MAX_PALETTES: usize = 16;

pub struct TiledIndexedImage {
    pub width_in_tiles: usize,
    pub height_in_tiles: usize,
    pub palettes: Vec<PaletteU3>,
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
        let palettes_u8: Vec<PaletteU8> = palette_tiles
            .iter()
            .map(|tiles| tiles_palette(tiles))
            .collect();
        let tiles = zip(tile_palette_indices, &source_image.tiles)
            .map(|(palette_index, tile)| {
                let palette = &palettes_u8[palette_index];
                let pattern = remap_tile(&tile, palette);
                IndexedTile {
                    palette_index: palette_index as u8,
                    pattern,
                }
            })
            .collect();
        let palettes = palettes_u8
            .iter()
            .map(|palette| palette.iter().copied().map(ColorU3::from).collect())
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
    let colors: Vec<ColorU8> = tile.iter().flatten().copied().collect();
    ColorDistribution::new(&colors)
}

fn tiles_palette(tiles: &[&Tile]) -> PaletteU8 {
    let colors: Vec<ColorU8> = tiles.iter().flat_map(|&tile| tile_colors(tile)).collect();
    let color_clustering = kmeans(MAX_PALETTE_COLORS, &colors, 100);
    color_clustering
        .centroids
        .iter()
        .map(|centroid| ColorU8 {
            r: centroid.0[0] as u8,
            g: centroid.0[1] as u8,
            b: centroid.0[2] as u8,
        })
        .collect()
}

fn tile_colors(tile: &Tile) -> Vec<ColorU8> {
    tile.iter().flatten().copied().collect()
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
        let palettes: Vec<PaletteU8> = source_image
            .palettes
            .iter()
            .map(|palette| palette.iter().copied().map(ColorU8::from).collect())
            .collect();
        let tiles = source_image
            .tiles
            .iter()
            .map(|tile| {
                tile_from_pattern_and_palette(&tile.pattern, &palettes[tile.palette_index as usize])
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

fn tile_from_pattern_and_palette(pattern: &IndexedPattern, palette: &[ColorU8]) -> Tile {
    let mut tile = [[ColorU8 { r: 0, g: 0, b: 0 }; TILE_SIZE]; TILE_SIZE];

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
