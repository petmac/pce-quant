use crate::{color::ColorU8, image::Image};

pub const TILE_SIZE: usize = 8;

pub struct TiledImage {
    pub width_in_tiles: usize,
    pub height_in_tiles: usize,
    pub tiles: Vec<Tile>,
}

impl From<Image> for TiledImage {
    fn from(source_image: Image) -> Self {
        let width_in_tiles = source_image.width / TILE_SIZE;
        let height_in_tiles = source_image.height / TILE_SIZE;
        let mut tiles = vec![
            [[ColorU8 {
                r: 255,
                g: 0,
                b: 255
            }; TILE_SIZE]; TILE_SIZE];
            width_in_tiles * height_in_tiles
        ];

        for y_px in 0..source_image.height {
            let y_tiles = y_px / TILE_SIZE;
            let y_px_in_tile = y_px % TILE_SIZE;
            let row_tiles_begin = y_tiles * width_in_tiles;
            let row_tiles_end = row_tiles_begin + width_in_tiles;
            let dest_row_tiles = &mut tiles[row_tiles_begin..row_tiles_end];
            let row_px_begin = y_px * source_image.width;
            let row_px_end = row_px_begin + source_image.width;
            let source_row_px = &source_image.pixels[row_px_begin..row_px_end];

            for x_px in 0..source_image.width {
                let x_tiles = x_px / TILE_SIZE;
                let x_px_in_tile = x_px % TILE_SIZE;
                let dest_tile = &mut dest_row_tiles[x_tiles];

                dest_tile[y_px_in_tile][x_px_in_tile] = source_row_px[x_px];
            }
        }

        TiledImage {
            width_in_tiles,
            height_in_tiles,
            tiles,
        }
    }
}

impl From<TiledImage> for Image {
    fn from(source_image: TiledImage) -> Image {
        let width = source_image.width_in_tiles * TILE_SIZE;
        let height = source_image.height_in_tiles * TILE_SIZE;
        let mut pixels = vec![
            ColorU8 {
                r: 255,
                g: 0,
                b: 255
            };
            width * height
        ];

        for y_px in 0..height {
            let y_tiles = y_px / TILE_SIZE;
            let y_px_in_tile = y_px % TILE_SIZE;
            let row_tiles_begin = y_tiles * source_image.width_in_tiles;
            let row_tiles_end = row_tiles_begin + source_image.width_in_tiles;
            let source_row_tiles = &source_image.tiles[row_tiles_begin..row_tiles_end];
            let row_px_begin = y_px * width;
            let row_px_end = row_px_begin + width;
            let dest_row_px = &mut pixels[row_px_begin..row_px_end];

            for x_px in 0..width {
                let x_tiles = x_px / TILE_SIZE;
                let x_px_in_tile = x_px % TILE_SIZE;
                let source_tile = &source_row_tiles[x_tiles];

                dest_row_px[x_px] = source_tile[y_px_in_tile][x_px_in_tile];
            }
        }

        Image {
            width,
            height,
            pixels,
        }
    }
}

pub type Tile = [[ColorU8; TILE_SIZE]; TILE_SIZE];
