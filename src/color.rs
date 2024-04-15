use clustering::Elem;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
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
