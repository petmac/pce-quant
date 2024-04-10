pub struct TrueColorImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<TrueColor>,
}

pub struct TrueColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TrueColor {
    pub fn new(rgb: &[u8]) -> Self {
        TrueColor {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}
