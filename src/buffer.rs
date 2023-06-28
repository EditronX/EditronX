use tui::buffer::Cell;

use crate::enums::{BufferType, Dimensions};

pub struct Buffer {
    pub size: Dimensions,
    pub is_float: bool,
    pub rows: Vec<Cell>,
    pub title: String,
    pub cursor: (usize, usize),
    pub offset: (usize, usize),
    pub buffer_type: BufferType,
    pub hidden: bool,
    pub pos: Dimensions,
}

impl Buffer {
    pub fn new(size: Dimensions, pos: Dimensions) -> Buffer {
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
