mod font;
mod util;

use iced::widget::{Column, container, image};
use iced::{Element, Padding};

use crate::font::{FontCache, read_images};

fn main() {
    let font = {
        let bytes = include_bytes!("../font-9x9.png");
        read_images(bytes)
    };

    let (filename, content) = util::read_file();
    let buffer = Buffer::from_str(&content);
    let state = State {
        buffer,
        font,
        filename,
    };

    iced::application(move || state.clone(), update, view).run();
}

#[derive(Debug, Default, Clone)]
pub struct Buffer {
    pub text: Vec<Vec<char>>,
    pub cursor: (usize, usize),
}

impl Buffer {
    pub fn from_str(s: &str) -> Self {
        let text = s
            .split("\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        Buffer {
            text,
            ..Buffer::default()
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    buffer: Buffer,
    font: FontCache,
    filename: Option<String>,
}

type Message = ();

fn update(_state: &mut State, _message: Message) {}

fn view(state: &State) -> Column<'_, Message> {
    let b = &state.buffer.text;
    let font = &state.font;

    let mut col_elems: Vec<Element<'_, Message>> = Vec::new();
    for line in b {
        let mut v: Vec<Element<'_, Message>> = Vec::new();
        for c in line {
            let c = c.to_ascii_lowercase();
            let Some(sym) = font.letters.get(&c) else {
                eprintln!("symbol {c} not in fontcache");
                std::process::exit(1);
            };

            let img = image(sym.img.clone()).filter_method(image::FilterMethod::Nearest);
            let img = container(img).padding(Padding::ZERO.left(2));
            v.push(img.into());
        }

        let row = iced::widget::row(v).padding(Padding::ZERO.top(2));
        col_elems.push(row.into());
    }

    iced::widget::column(col_elems)
}
