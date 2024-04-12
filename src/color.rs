#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorU8 {
    pub fn new(rgb: &[u8]) -> ColorU8 {
        ColorU8 {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ColorU3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<ColorU8> for ColorU3 {
    fn from(value: ColorU8) -> Self {
        ColorU3 {
            r: value.r >> 5,
            g: value.g >> 5,
            b: value.b >> 5,
        }
    }
}

impl Into<ColorU8> for ColorU3 {
    fn into(self) -> ColorU8 {
        let r = u3_to_u8(self.r);
        let g = u3_to_u8(self.g);
        let b = u3_to_u8(self.b);
        ColorU8 { r, g, b }
    }
}

impl Into<[u8; 3]> for ColorU3 {
    fn into(self) -> [u8; 3] {
        let r = u3_to_u8(self.r);
        let g = u3_to_u8(self.g);
        let b = u3_to_u8(self.b);
        [r, g, b]
    }
}

fn u3_to_u8(x: u8) -> u8 {
    // 765----- Shift left by 5
    // ---432-- Shift left by 2
    // ------10 Shift right by 1
    (x << 5) | (x << 2) | (x >> 1)
}

#[derive()]
pub struct ColorF {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Into<ColorU3> for ColorF {
    fn into(self) -> ColorU3 {
        let r = self.r.round() as u8 >> 5;
        let g = self.g.round() as u8 >> 5;
        let b = self.b.round() as u8 >> 5;
        ColorU3 { r, g, b }
    }
}
