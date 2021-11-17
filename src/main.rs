use std::io::stdout;

fn main() {
    let arg_matches = args().get_matches();

    let img_path = arg_matches.value_of("file")
        .expect("Bad implementation, a path should be present");

    let img = image::open(img_path)
        .expect("An image should be present at the location");

    // let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));

    let conf = Default::default();
    let (_width, height) = viuer::print(&img, &conf)
        .expect("Image printing failed");

    crossterm::execute!(stdout(),
        crossterm::cursor::MoveUp(height.try_into().unwrap()),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown))
        .unwrap();

    viuer::print(&img, &conf)
        .expect("Image printing failed");
}

fn args<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(clap::Arg::with_name("file")
            .help("Image location")
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
