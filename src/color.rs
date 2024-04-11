#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb8 {
    pub fn new(rgb: &[u8]) -> Self {
        Rgb8 {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}
