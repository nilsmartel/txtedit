use std::collections::BTreeMap;

use image::GenericImageView;

pub struct FontCache {
    // TODO better use an array
    letters: BTreeMap<char, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    symbol: char,
    width: u32,
    height: u32,
    img: iced::widget::image::Handle,
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
pub fn read_images(bytes: &[u8]) -> FontCache {
    

    let png = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .expect("decode png image");

    let img = png.as_rgb8().expect("convert rgba8");

    let _w = img.width();
    let _w = img.height();

    let mut letters = BTreeMap::new();
    for (row, coordinate_str) in SIGNMAP.iter().enumerate() {
        let row = row as u32;
        for (col, symbol) in coordinate_str.chars().enumerate() {
            let col = col as u32;
            let sprite = extract_sprite(symbol, img.view(row * GRID, col * GRID, GRID, GRID));

            letters.insert(symbol, sprite);
        }
    }

    FontCache { letters }
}

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
    let right_cutoff = (0..GRID)
        .rev()
        .map(is_white_column)
        .take_while(|&x| x)
        .count() as u32;

    let width = GRID - left_cutoff - right_cutoff;

    let mut pixels: Vec<u8> = Vec::with_capacity(height as usize + width as usize * 4);
    for y in 0..height  {
        let y = y as u32;
        for x in 0..width  {
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
