mod font;
mod util;

use iced::widget::Column;

use crate::{font::read_images, util::str_to_buffer};

fn main() {
    let _font_cache = {
        let bytes = include_bytes!("../font-9x9.png");
        read_images(bytes)
    };

    let (_filename, content) = util::read_file();
    let buffer = Buffer {
        lines: str_to_buffer(content),
        ..Buffer::default()
    };
    let state = State { buffer };

    iced::application(|| state.clone(), update, view);
}

#[derive(Debug, Default, Clone)]
pub struct Buffer {
    pub lines: Vec<Vec<u16>>,
    pub cursor: (usize, usize),
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
