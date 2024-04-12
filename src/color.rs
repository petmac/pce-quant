#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(rgb: &[u8]) -> Self {
        Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}
