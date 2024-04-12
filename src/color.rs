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

#[derive()]
pub struct ColorF {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Into<Color> for ColorF {
    fn into(self) -> Color {
        let r = self.r.round() as u8;
        let g = self.g.round() as u8;
        let b = self.b.round() as u8;
        Color { r, g, b }
    }
}
