use image::{Pixel, Rgba};

pub(crate) struct PixelColors {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl PixelColors {
    const RED_INDEX: usize = 0;
    const GREEN_INDEX: usize = 1;
    const BLUE_INDEX: usize = 2;
    const ALPHA_INDEX: usize = 3;

    pub fn new(rgba: &Rgba<u8>) -> PixelColors {
        Self {
            red: rgba.channels()[PixelColors::RED_INDEX],
            green: rgba.channels()[PixelColors::GREEN_INDEX],
            blue: rgba.channels()[PixelColors::BLUE_INDEX],
            alpha: rgba.channels()[PixelColors::ALPHA_INDEX]
        }
    }
}
