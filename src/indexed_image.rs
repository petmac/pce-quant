use crate::palette::Palette;

pub struct IndexedImage {
    pub width: usize,
    pub height: usize,
    pub palette: Palette,
    pub pixels: Vec<u8>,
}
