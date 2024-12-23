use crate::color::Color;

// Color index 0 is reserved for the background, so don't include it
// Technically the first palette could use all 16 colours, but that sounds quite hard to get working
pub const MAX_PALETTE_COLORS: usize = 15;

pub type Palette = Vec<Color>;
