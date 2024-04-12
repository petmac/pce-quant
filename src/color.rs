#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorU8 {
    pub fn new(rgb: &[u8]) -> Self {
        ColorU8 {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}

#[derive()]
pub struct ColorF {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Into<ColorU8> for ColorF {
    fn into(self) -> ColorU8 {
        let r = self.r.round() as u8;
        let g = self.g.round() as u8;
        let b = self.b.round() as u8;
        ColorU8 { r, g, b }
    }
}
