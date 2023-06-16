use crate::enums::{BufferType, Size};

pub struct Buffer {
    pub size: Size,
    pub is_float: bool,
    pub rows: Vec<String>,
    pub title: String,
    pub cursor: (usize, usize),
    pub offset: (usize, usize),
    pub buffer_type: BufferType,
    pub hidden: bool,
    pub pos: (usize, usize),
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            size: Size::Absolute(0, 0),
            pos: (0, 0),
            is_float: false,
            rows: Vec::new(),
            title: String::from("[No Name]"),
            cursor: (0, 0),
            offset: (0, 0),
            buffer_type: BufferType::Buffer,
            hidden: false,
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }
}
