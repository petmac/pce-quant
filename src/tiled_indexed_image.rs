use std::iter::zip;

use crate::{
    bsp::BspTree,
    color::{ColorU3, ColorU8},
    distribution::Distribution,
    palette::{PaletteU3, PaletteU8},
    remap::{nearest_color_in_palette, remap},
    tiled_image::{Tile, TiledImage, TILE_SIZE},
};

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
            .map(|tiles| tiles_color_distribution(tiles))
            .map(BspTree::new)
            .map(build_palette)
            .collect();
        let tiles = zip(tile_palette_indices, &source_image.tiles)
            .map(|(palette_index, tile)| {
                let palette = &palettes_u8[palette_index as usize];
                let pattern = remap_tile(&tile, palette);
                IndexedTile {
                    palette_index,
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

fn tile_palette_indices(tiles: &[Tile]) -> Vec<u8> {
    let tile_average_colors: Vec<ColorU8> = tiles
        .iter()
        .map(tile_color_distribution)
        .map(|distribution| distribution.average_color())
        .map(ColorU8::from)
        .collect();
    let distribution = Distribution::new(&tile_average_colors);
    let tree = BspTree::new(distribution);
    let palette = build_palette(tree);
    let tile_palette_indices = remap(&tile_average_colors, &palette);
    tile_palette_indices
}

fn tile_color_distribution(tile: &Tile) -> Distribution {
    let colors: Vec<ColorU8> = tile.iter().flatten().copied().collect();
    Distribution::new(&colors)
}

fn build_palette(tree: BspTree) -> PaletteU8 {
    tree.leaves
        .iter()
        .map(Distribution::average_color)
        .map(ColorU8::from)
        .collect()
}

fn palette_tile_indices(tile_palette_indices: &[u8]) -> Vec<Vec<usize>> {
    let max_palette_index = tile_palette_indices.iter().copied().max();
    let palette_count = max_palette_index.unwrap_or(0) as usize + 1;
    let mut palette_tile_indices = vec![Vec::new(); palette_count];

    for (tile_index, &tile_palette_index) in tile_palette_indices.iter().enumerate() {
        palette_tile_indices[tile_palette_index as usize].push(tile_index);
    }

    palette_tile_indices
}

fn tiles_color_distribution(tiles: &[&Tile]) -> Distribution {
    let colors: Vec<ColorU8> = tiles
        .iter()
        .map(|&tile| tile.iter().flatten())
        .flatten()
        .copied()
        .collect();
    Distribution::new(&colors)
}

fn remap_tile(ideal_tile: &Tile, palette: &[ColorU8]) -> IndexedPattern {
    let mut pattern = IndexedPattern::default();

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            pattern[y][x] = nearest_color_in_palette(&ideal_tile[y][x], palette) as u8;
        }
    }

    pattern
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

fn tile_from_pattern_and_palette(pattern: &IndexedPattern, palette: &[ColorU8]) -> Tile {
    let mut tile = [[ColorU8 { r: 0, g: 0, b: 0 }; TILE_SIZE]; TILE_SIZE];

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let color_index = pattern[y][x] as usize;
            let color = palette[color_index];
            tile[y][x] = color;
        }
    }

    tile
}
