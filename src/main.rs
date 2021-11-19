mod seam_carving;
mod pixel_colors;

use std::cmp::max;
use image::{DynamicImage, GenericImageView};
use std::default::Default;
use viuer::Config;

fn main() {
    let arg_matches = args().get_matches();

    let img_path = arg_matches.value_of("file")
        .expect("Bad implementation, a path should be present");

    let mut img = image::open(img_path)
        .expect(format!("No image at '{}'", img_path).as_str());

    img = adjust_width( arg_matches.value_of("width").unwrap(), img);
    adjust_height( arg_matches.value_of("height").unwrap(), img);
}

fn adjust_width(arg_width: &str, mut img: DynamicImage) -> DynamicImage {
    let arg_width: u32 = arg_width.parse().unwrap();
    let to_width = max(1, (img.width() * arg_width) / 100);
    let delta_width = img.width() - to_width;

    let mut conf = Config {
        height: Some(img.height()),
        width: Some(img.width()),
        ..Default::default()
    };

    for _ in 0..delta_width {
        // Improvement idea: we don't need to be computing the energy map from scratch
        // If only a pixel is removed from each column, then we only need to compute the energy from
        // that area
        let energy_map = seam_carving::calculate_energy_map_width(&img);

        let seam = seam_carving::find_low_energy_seam_width(&energy_map);

        // Good for debugging purposes
        seam_carving::print_energy_map_width(&energy_map, &conf, &Some(&seam));

        img = seam_carving::delete_seam_width(img, &seam);
        conf.width = Some(conf.width.unwrap() - 1);

        viuer::print(&img, &conf)
            .expect("Image printing failed");
    }

    img
}

fn adjust_height(arg_height: &str, mut img: DynamicImage) -> DynamicImage {
    let arg_height: u32 = arg_height.parse().unwrap();
    let to_height = max(1, (img.height() * arg_height) / 100);
    let delta_height = img.height() - to_height;

    let mut conf = Config {
        height: Some(img.height()),
        width: Some(img.width()),
        ..Default::default()
    };

    for _ in 0..delta_height {
        // Improvement idea: we don't need to be computing the energy map from scratch
        // If only a pixel is removed from each row, then we only need to compute the energy from
        // that area
        let energy_map = seam_carving::calculate_energy_map_height(&img);

        let seam = seam_carving::find_low_energy_seam_height(&energy_map);

        // Good for debugging purposes
        seam_carving::print_energy_map_height(&energy_map, &conf, &Some(&seam));

        img = seam_carving::delete_seam_height(img, &seam);
        conf.height = Some(conf.height.unwrap() - 1);

        viuer::print(&img, &conf)
            .expect("Image printing failed");
    }

    img
}

fn args<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(clap::Arg::with_name("file")
            .help("Image location (no tilde expansion supported)")
            .long("file")
            .required(true)
            .takes_value(true))
        .arg(clap::Arg::with_name("width")
            .help("Set width percentage (between 1 and 100)")
            .long("width")
            .default_value("100")
            .validator(|arg| {
                let percentage: u8 = arg.parse().expect("Width is not a number fitting u8");
                if percentage >= 1 && percentage <= 100 {
                    Ok(())
                } else {
                    Err(String::from("Width must be between 1 and 100"))
                }
            }))
        .arg(clap::Arg::with_name("height")
            .help("Set height percentage (between 1 and 100)")
            .long("height")
            .default_value("100")
            .validator(|arg| {
                let percentage: u8 = arg.parse().expect("Height is not a number fitting u8");
                if percentage >= 1 && percentage <= 100 {
                    Ok(())
                } else {
                    Err(String::from("Height must be between 1 and 100"))
                }
            }))
}
