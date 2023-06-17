use tui::buffer::Cell;

use crate::enums::{BufferType, Size};

pub struct Buffer {
    pub size: Size,
    pub is_float: bool,
    pub rows: Vec<Cell>,
    pub title: String,
    pub cursor: (usize, usize),
    pub offset: (usize, usize),
    pub buffer_type: BufferType,
    pub hidden: bool,
    pub pos: (usize, usize),
}

impl Buffer {
    pub fn new(size: Size, pos: (usize, usize)) -> Buffer {
        Buffer {
            size,
            pos,
            is_float: false,
            rows: Vec::new(),
            title: String::from("[No Name]"),
            cursor: (0, 0),
            offset: (0, 0),
            buffer_type: BufferType::Buffer,
            hidden: false,
        }
    }
}
