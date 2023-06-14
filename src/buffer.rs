use crate::enums::BufferType;

pub struct Buffer {
    size: (usize, usize),
    is_float: bool,
    rows: Vec<String>,
    title: String,
    cursor: (usize, usize),
    offset: (usize, usize),
    buffer_type: BufferType,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            size: (0, 0),
            is_float: false,
            rows: Vec::new(),
            title: String::new(),
            cursor: (0, 0),
            offset: (0, 0),
            buffer_type: BufferType::Buffer,
        }
    }
}
