use std::cmp::max;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, Rgba};
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

pub fn find_low_energy_seam_width(energy_map: &Vec<Vec<i32>>) -> Vec<usize> {
    let height = energy_map.len();
    let width = energy_map[0].len();

    let mut seam_energies = vec![vec![0 as i32; width]; height];
    for w in 0..width {
        seam_energies[0][w] = energy_map[0][w];
    }
    for h in 1..height {
        for w in 0..width {
            let mut energy = seam_energies[h-1][w];
            if w > 0 && seam_energies[h-1][w-1] < energy {
                energy = seam_energies[h-1][w-1];
            }
            if w < width-1 && seam_energies[h-1][w+1] < energy {
                energy = seam_energies[h-1][w+1];
            }

            seam_energies[h][w] = energy_map[h][w] + energy;
        }
    }

    let mut min_start_w_idx = 0;
    for w in 1..width {
        if seam_energies[height-1][w] < seam_energies[height-1][min_start_w_idx] {
            min_start_w_idx = w;
        }
    }

    // The index represents the height
    // The value represents the index of the pixel we want to delete
    let mut seam_low_energy_path = vec![min_start_w_idx; height];
    for h in (0..height-1).rev() {
        let prev_w = seam_low_energy_path[h+1];

        let mut idx = prev_w;
        if prev_w > 0 && seam_energies[h][prev_w-1] < seam_energies[h][idx] {
            idx = prev_w - 1;
        }
        if prev_w < width-1 && seam_energies[h][prev_w+1] < seam_energies[h][idx] {
            idx = prev_w + 1;
        }

        seam_low_energy_path[h] = idx;
    }

    seam_low_energy_path
}

pub fn delete_seam_width(img: DynamicImage, seam: &Vec<usize>) -> DynamicImage {
    let mut img = img;

    for h in 0..img.height() {
        for w in (seam[h as usize] as u32)..img.width()-1 {
            img.put_pixel(w, h, img.get_pixel(w+1, h));
        }
    }

    img.crop_imm(0, 0, img.width()-1, img.height())
}

pub fn print_energy_map_width(energy_map: &Vec<Vec<i32>>, config: &Config, seam: &Option<&Vec<usize>>) {
    if let Some(seam) = seam {
        let mut xy_seam = vec![(0,0); seam.len()];
        for i in 0..seam.len() {
            xy_seam[i] = (i, seam[i]);
        }
        print_energy_map(energy_map, config, &Some(&xy_seam));
    } else {
        print_energy_map(energy_map, config, &None);
    }
}

pub fn calculate_energy_map_height(img: &image::DynamicImage) -> Vec<Vec<i32>> {
    let mut energy_map = vec![vec![0; img.width() as usize]; img.height() as usize];

    for height in 0..img.height() {
        for width in 0..img.width() {
            let up = if height == 0 { None } else { Some(img.get_pixel(width, height - 1)) };
            let middle = img.get_pixel(width, height);
            let down = if height == img.height()-1 { None } else { Some(img.get_pixel(width, height + 1)) };

            energy_map[height as usize][width as usize] = pixel_energy(&up, &middle, &down);
        }
    }

    energy_map
}

pub fn find_low_energy_seam_height(energy_map: &Vec<Vec<i32>>) -> Vec<usize> {
    let height = energy_map.len();
    let width = energy_map[0].len();

    let mut seam_energies = vec![vec![0 as i32; width]; height];
    for h in 0..height {
        seam_energies[h][0] = energy_map[h][0];
    }
    for w in 1..width {
        for h in 0..height {
            let mut energy = seam_energies[h][w-1];
            if h > 0 && seam_energies[h-1][w-1] < energy {
                energy = seam_energies[h-1][w-1];
            }
            if h < height-1 && seam_energies[h+1][w-1] < energy {
                energy = seam_energies[h+1][w-1];
            }

            seam_energies[h][w] = energy_map[h][w] + energy;
        }
    }

    let mut min_start_h_idx = 0;
    for h in 1..height {
        if seam_energies[h][width-1] < seam_energies[min_start_h_idx][width-1] {
            min_start_h_idx = h;
        }
    }

    // The index represents the width
    // The value represents the index of the pixel we want to delete
    let mut seam_low_energy_path = vec![min_start_h_idx; width];
    for w in (0..width-1).rev() {
        let prev_h = seam_low_energy_path[w+1];

        let mut idx = prev_h;
        if prev_h > 0 && seam_energies[prev_h-1][w] < seam_energies[idx][w] {
            idx = prev_h - 1;
        }
        if prev_h < height-1 && seam_energies[prev_h+1][w] < seam_energies[idx][w] {
            idx = prev_h + 1;
        }

        seam_low_energy_path[w] = idx;
    }

    seam_low_energy_path
}

pub fn delete_seam_height(img: DynamicImage, seam: &Vec<usize>) -> DynamicImage {
    let mut img = img;

    for w in 0..img.width() {
        for h in (seam[w as usize] as u32)..img.height()-1 {
            img.put_pixel(w, h, img.get_pixel(w, h+1));
        }
    }

    img.crop_imm(0, 0, img.width(), img.height()-1)
}

pub fn print_energy_map_height(energy_map: &Vec<Vec<i32>>, config: &Config, seam: &Option<&Vec<usize>>) {
    if let Some(seam) = seam {
        let mut xy_seam = vec![(0,0); seam.len()];
        for i in 0..seam.len() {
            xy_seam[i] = (seam[i], i);
        }
        print_energy_map(energy_map, config, &Some(&xy_seam));
    } else {
        print_energy_map(energy_map, config, &None);
    }
}

fn print_energy_map(energy_map: &Vec<Vec<i32>>, config: &Config, seam: &Option<&Vec<(usize,usize)>>) {
    let height = energy_map.len();
    let width = energy_map[0].len();

    let mut max_energy = 0;
    for h in 0..height {
        for w in 0..width {
            max_energy = max(max_energy, energy_map[h][w]);
        }
    }

    let mut img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let scale: f32 = (energy_map[y as usize][x as usize] as f32) / (max_energy as f32);
        let color: u8 = (scale * (u8::MAX as f32)) as u8;
        Luma([color])
    });

    if let Some(seam) = seam {
        for i in 0..seam.len() {
            // u8::MAX == White color
            img.put_pixel(seam[i].1 as u32, seam[i].0 as u32, Luma([u8::MAX]));
        }
    }

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

        // If we want to have a better preview of the energy map, then the square root
        // helps normalise the values
        // ((left_energy + right_energy) as f32).sqrt() as i32
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
