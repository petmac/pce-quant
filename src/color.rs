use clustering::Elem;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn truncate_to_3_bits(self) -> Color {
        Color {
            r: truncate_to_3_bits(self.r),
            g: truncate_to_3_bits(self.g),
            b: truncate_to_3_bits(self.b),
        }
    }
}

impl Elem for Color {
    fn dimensions(&self) -> usize {
        3
    }

    fn at(&self, i: usize) -> f64 {
        match i {
            0 => self.r,
            1 => self.g,
            _ => self.b,
        }
    }
}

fn truncate_to_3_bits(x: f64) -> f64 {
    (x * 7.0).round() / 7.0
}
