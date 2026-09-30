use std::{collections::BTreeMap, io::Write};

use image::GenericImageView;

#[derive(Debug, Clone)]
pub struct FontCache {
    // TODO better use an array
    pub letters: BTreeMap<char, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol: char,
    pub width: u32,
    pub height: u32,
    pub img: iced::widget::image::Handle,
}

const SIGNMAP: [&'static str; 9] = [
    "abcdefghijklmnopqrstuvwxyz",
    "ä             ö   ß ü",
    "0123456789",
    "+-*/",
    ".,-",
    ";:_",
    "<>  {}[]",
    "!\"§$%&/()=?`'#",
    "\\",
];

const GRID: u32 = 9;
/// Build a fontcache by extracting the pixels from the png one by one.
pub fn read_images(bytes: &[u8]) -> FontCache {
    let png = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .expect("decode png image");

    let img = png.as_rgb8().expect("convert rgba8");

    let mut letters = BTreeMap::new();
    letters.insert(' ', white_symbol(' ', 4, GRID));
    // Go over each symbol in out signmap (e.g. corresponding chars to how they appear inside the pngs grid.)
    for (y_pos, coordinate_str) in SIGNMAP.iter().enumerate() {
        let y_pos = y_pos as u32;
        for (x_pos, symbol) in coordinate_str.chars().enumerate() {
            if symbol == ' ' {
                continue;
            }
            let x_pos = x_pos as u32;
            let sprite = extract_sprite(symbol, img.view(x_pos * GRID, y_pos * GRID, GRID, GRID));

            letters.insert(symbol, sprite);
        }
    }

    FontCache { letters }
}

/// Trim the left and right sides of an 9x9 grid image of a letter.
fn extract_sprite(
    symbol: char,
    img: image::SubImage<&image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>,
) -> Symbol {
    let height = GRID;

    // We expect this to be GRID*GRID in size.
    // We check where we can cut of left and right.
    // Boxing in the available pixels

    let is_white_column = |x| // for each column from the left
        // check pixels top to bottom if they are white (== empty)
        (0..height).all(|y| img.get_pixel(x, y).0 == [255, 255, 255]);

    let left_cutoff = (0..GRID).map(is_white_column).take_while(|&x| x).count() as u32;
    // If the section is all white (not yet drawn)
    // respond with an "unknown" symbol
    if left_cutoff == 9 {
        eprintln!("symbol for {symbol} is empty");
        return unknown_symbol(symbol, 5, height);
    }
    let right_cutoff = (0..GRID)
        .rev()
        .map(is_white_column)
        .take_while(|&x| x)
        .count() as u32;

    let width = GRID - left_cutoff - right_cutoff;

    let mut pixels: Vec<u8> = Vec::with_capacity(height as usize + width as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let x = x + left_cutoff;

            let p = img.get_pixel(x, y).0;
            pixels.push(p[0]);
            pixels.push(p[1]);
            pixels.push(p[2]);
            pixels.push(255);
        }
    }

    let img = iced::widget::image::Handle::from_rgba(width, height, pixels);

    Symbol {
        symbol,
        width,
        height,
        img,
    }
}

fn unknown_symbol(symbol: char, width: u32, height: u32) -> Symbol {
    let black = [0, 0, 0, 255];
    let white = [255, 255, 255, 255];
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let v = if ((x + y) & 1) == 1 { white } else { black };
            pixels.write_all(&v).expect("write pixels");
        }
    }

    let img = iced::widget::image::Handle::from_rgba(width, height, pixels);
    Symbol {
        symbol,
        width,
        height,
        img,
    }
}

fn white_symbol(symbol: char, width: u32, height: u32) -> Symbol {
    let white = [255, 255, 255, 255];
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            pixels.write_all(&white).expect("write pixels");
        }
    }

    let img = iced::widget::image::Handle::from_rgba(width, height, pixels);
    Symbol {
        symbol,
        width,
        height,
        img,
    }
}
