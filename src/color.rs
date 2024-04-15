use clustering::Elem;

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

impl From<ColorU8> for ColorU3 {
    fn from(value: ColorU8) -> Self {
        ColorU3 {
            r: value.r >> 5,
            g: value.g >> 5,
            b: value.b >> 5,
        }
    }
}

impl Elem for ColorU8 {
    fn dimensions(&self) -> usize {
        3
    }

    fn at(&self, i: usize) -> f64 {
        match i {
            0 => self.r as f64,
            1 => self.g as f64,
            _ => self.b as f64,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ColorU3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<ColorU3> for [u8; 3] {
    fn from(value: ColorU3) -> [u8; 3] {
        let color_u8 = ColorU8::from(value);
        [color_u8.r, color_u8.g, color_u8.b]
    }
}

impl From<ColorU3> for ColorU8 {
    fn from(value: ColorU3) -> ColorU8 {
        let r = u3_to_u8(value.r);
        let g = u3_to_u8(value.g);
        let b = u3_to_u8(value.b);
        ColorU8 { r, g, b }
    }
}

fn u3_to_u8(x: u8) -> u8 {
    // 765----- Shift left by 5
    // ---432-- Shift left by 2
    // ------10 Shift right by 1
    (x << 5) | (x << 2) | (x >> 1)
}

#[derive(Clone, Copy)]
pub struct ColorF {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl From<ColorF> for ColorU3 {
    fn from(value: ColorF) -> ColorU3 {
        ColorU3::from(ColorU8::from(value))
    }
}

impl From<ColorF> for ColorU8 {
    fn from(value: ColorF) -> ColorU8 {
        let r = value.r.round() as u8;
        let g = value.g.round() as u8;
        let b = value.b.round() as u8;
        ColorU8 { r, g, b }
    }
}
