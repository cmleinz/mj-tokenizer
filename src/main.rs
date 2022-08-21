use clap::Parser;
use core::fmt;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};
use std::{fmt::format, fs::File, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Location of midjourney tile image.
    #[clap(value_parser)]
    file: String,

    /// The tile number you would like to tokenize
    #[clap(short, long, value_parser)]
    tile: Option<u8>,

    /// The size of the final token, both width and height (pixels)
    #[clap(short, long, value_parser, default_value_t = 256)]
    size: u32,

    /// The frame that the final token will use (see assets folder)
    #[clap(short, long, value_parser, default_value_t = 1)]
    frame: u8,
}

#[derive(Debug)]
enum ImageError {
    InvalidTileNumber,
    InvalidFrame,
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match &self {
            ImageError::InvalidTileNumber => "Invalid tile number. You must select 1 - 4, or leave this blank for upscaled images",
            ImageError::InvalidFrame => "Invalid frame. We searched for border-.png in the assets folder but found nothing.",
        };
        write!(f, "{}", err)
    }
}

impl std::error::Error for ImageError {}

enum MidJourneyTiles {
    Full, // For upscaled images
    One,
    Two,
    Three,
    Four,
}

impl MidJourneyTiles {
    fn from_u8(input: Option<u8>) -> Result<Self, ImageError> {
        match input {
            None => Ok(MidJourneyTiles::Full),
            Some(1) => Ok(MidJourneyTiles::One),
            Some(2) => Ok(MidJourneyTiles::Two),
            Some(3) => Ok(MidJourneyTiles::Three),
            Some(4) => Ok(MidJourneyTiles::Four),
            Some(_) => Err(ImageError::InvalidTileNumber),
        }
    }
}

fn get_tile_coordinates(tile: MidJourneyTiles, img_width: u32, img_height: u32) -> [u32; 4] {
    // If the images are not square this will currently crop the right side, rather than splitting the difference
    let half_width = img_width / 2;
    let half_height = img_height / 2;
    let half = std::cmp::min(half_height, half_width);
    let zero = 0 as u32;

    match tile {
        MidJourneyTiles::Full => [zero, zero, img_width, img_height],
        MidJourneyTiles::One => [zero, zero, half, half],
        MidJourneyTiles::Two => [half, zero, half, half],
        MidJourneyTiles::Three => [zero, half_height, half, half],
        MidJourneyTiles::Four => [half, half, half, half],
    }
}

fn tokenize(
    img: &mut image::DynamicImage,
    tile: MidJourneyTiles,
    size: u32,
    border: Option<DynamicImage>,
) {
    let coords = get_tile_coordinates(tile, img.width(), img.height());
    let cropped_img = img.crop(coords[0], coords[1], coords[2], coords[3]);
    let cropped_img = cropped_img.resize(size, size, FilterType::Lanczos3);
    let mut rgba_img = cropped_img.into_rgba16();
    let half = std::cmp::min(size / 2, size / 2);
    // Add a buffer so the image doesn't extend out to the edges. This is necessary since most frames
    let buffer = size / 50;
    for (x, y, rgba) in rgba_img.enumerate_pixels_mut() {
        let x_ = x as i32 - half as i32;
        let y_ = y as i32 - half as i32;
        let xy_squared = x_ * x_ + y_ * y_;
        let r = (xy_squared as f64).sqrt() as u32 + 1;
        if r + buffer >= half {
            rgba[3] = 0;
        }
    }
    if let Some(b) = border {
        let border = b.into_rgba16();
        image::imageops::overlay(&mut rgba_img, &border, 0, 0);
    }
    match rgba_img.save_with_format("Token.png", ImageFormat::Png) {
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}", e),
    }
}

fn get_border(border: u8, size: u32) -> Result<DynamicImage, ImageError> {
    let image = match image::open(&format!("assets/border-{}.png", border)) {
        Ok(i) => i,
        Err(e) => {
            println!("{e}");
            return Err(ImageError::InvalidFrame);
        }
    };
    Ok(image.resize(size, size, FilterType::Lanczos3))
}

fn main() {
    let cli = Args::parse();
    let tile = match MidJourneyTiles::from_u8(cli.tile) {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    let mut img = match image::open(cli.file) {
        Ok(i) => i,
        Err(_) => {
            println!("Unable to read the designated file. Check the location and try again.");
            return;
        }
    };
    let frame = match cli.frame {
        0 => None,
        i => match get_border(i, cli.size) {
            Ok(i) => Some(i),
            Err(e) => {
                println!("{e}");
                return;
            }
        },
    };

    tokenize(&mut img, tile, cli.size, frame);
}
