mod font;
mod util;

use iced::widget::Column;

use crate::font::read_images;

fn main() {
    let _font_cache = {
        let bytes = include_bytes!("../font-9x9.png");
        read_images(bytes)
    };

    let (_filename, content) = util::read_file();
    let buffer = Buffer::from_str(&content);
    let state = State { buffer };

    iced::application(|| state.clone(), update, view);
}

#[derive(Debug, Default, Clone)]
pub struct Buffer {
    pub lines: Vec<Vec<char>>,
    pub cursor: (usize, usize),
}

impl Buffer {
    pub fn from_str(s: &str) -> Self {
        let lines = s
            .split("\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        Buffer {
            lines,
            ..Buffer::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    buffer: Buffer,
}

type Message = ();

fn update(_state: &mut State, _message: Message) {}

fn view(_state: &State) -> Column<'_, Message> {
    todo!()
}
