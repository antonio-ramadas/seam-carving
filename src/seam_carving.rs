use std::cmp::max;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgba};
use viuer::Config;
use crate::pixel_colors::PixelColors;

pub fn calculate_energy_map_width(img: &image::DynamicImage) -> Vec<Vec<i32>> {
    let mut energy_map = vec![vec![0; img.width() as usize]; img.height() as usize];

    for height in 0..img.height() {
        for width in 0..img.width() {
            let left = if width == 0 { None } else { Some(img.get_pixel(width - 1, height)) };
            let middle = img.get_pixel(width, height);
            let right = if width == img.width()-1 { None } else { Some(img.get_pixel(width + 1, height)) };

            energy_map[height as usize][width as usize] = pixel_energy(&left, &middle, &right);
        }
    }

    energy_map
}

pub fn print_energy_map(energy_map: Vec<Vec<i32>>, config: &Config) {
    let height = energy_map.len();
    let width = energy_map[0].len();

    let mut max_energy = 1;
    for h in 0..height {
        for w in 0..width {
            max_energy = max(max_energy, energy_map[h][w]);
        }
    }

    let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let scale: f32 = (energy_map[y as usize][x as usize] as f32) / (max_energy as f32);
        let color: u8 = (scale * (u8::MAX as f32)) as u8;
        Luma([color])
    });

    viuer::print(&DynamicImage::ImageLuma8(img), config)
        .expect("Error printing energy map");
}

fn pixel_energy(left: &Option<Rgba<u8>>, middle: &Rgba<u8>, right: &Option<Rgba<u8>>) -> i32 {
    const ALPHA_DELETE_THRESHOLD: u8 = 244;
    const PIXEL_DELETE_ENERGY: i32 = -1;

    let middle_colors = PixelColors::new(middle);

    let left_energy = energy(left, &middle_colors);
    let right_energy = energy(right, &middle_colors);

    if middle_colors.alpha > ALPHA_DELETE_THRESHOLD {
        // No need to do the square root if it applies to all the operations
        left_energy + right_energy
    } else {
        PIXEL_DELETE_ENERGY
    }
}

fn energy(neighbor: &Option<Rgba<u8>>, middle_colors: &PixelColors) -> i32 {
    match neighbor {
        None => 0,
        Some(neighbor_pixel) => {
            let neighbor_colors = PixelColors::new(neighbor_pixel);

            (neighbor_colors.red as i32 - middle_colors.red as i32).pow(2) +
                (neighbor_colors.green as i32 - middle_colors.green as i32).pow(2) +
                (neighbor_colors.blue as i32 - middle_colors.blue as i32).pow(2)
        }
    }
}
