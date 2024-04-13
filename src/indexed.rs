use crate::color::ColorU3;

pub struct IndexedImage {
    pub width: usize,
    pub height: usize,
    pub palette: Vec<ColorU3>,
    pub pixels: Vec<u8>,
}
