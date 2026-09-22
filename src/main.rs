mod util;

use iced::widget::{Column, toggler::default};

use crate::util::str_to_buffer;

fn main() {
    let (filename, content) = util::read_file();
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

fn update(state: &mut State, message: Message) {}

fn view(state: &State) -> Column<Message> {
    todo!()
}
