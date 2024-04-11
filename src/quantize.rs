use crate::{color::Rgb8, indexed::IndexedImage, true_color::TrueColorImage};

pub fn quantize(input_image: &TrueColorImage) -> IndexedImage {
    let palette: Vec<Rgb8> = input_image.pixels[0..16].iter().copied().collect();
    let pixels = vec![0; input_image.pixels.len()];

    IndexedImage {
        width: input_image.width,
        height: input_image.height,
        palette,
        pixels,
    }
}
