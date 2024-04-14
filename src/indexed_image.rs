use crate::{
    bsp::BspTree,
    color::{ColorU3, ColorU8},
    distribution::Distribution,
    image::Image,
    palette::PaletteU3,
    remap::remap,
};

pub struct IndexedImage {
    pub width: usize,
    pub height: usize,
    pub palette: PaletteU3,
    pub pixels: Vec<u8>,
}

impl From<Image> for IndexedImage {
    fn from(input_image: Image) -> IndexedImage {
        let distribution = Distribution::new(&input_image.pixels);
        let tree = BspTree::new(distribution);
        let palette = build_palette(tree);
        let palette_u8: Vec<ColorU8> = palette.iter().copied().map(ColorU8::from).collect();
        let pixels = remap(&input_image.pixels, &palette_u8);

        IndexedImage {
            width: input_image.width,
            height: input_image.height,
            palette,
            pixels,
        }
    }
}

fn build_palette(tree: BspTree) -> PaletteU3 {
    tree.leaves
        .iter()
        .map(Distribution::average_color)
        .map(ColorU3::from)
        .collect()
}
