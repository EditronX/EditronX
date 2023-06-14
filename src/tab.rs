use crate::buffer::Buffer;

pub struct Tab {
    buflist: Vec<Buffer>,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            buflist: vec![Buffer::new()],
        }
    }
}
