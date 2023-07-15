use crate::{buffer::Buffer, enums::SplitVert};

pub struct Tab {
    pub buflist: Vec<Buffer>,
    pub active_buf: usize,
    tab_size: (usize, usize),
}

impl Tab {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buflist: vec![Buffer::new((width, height), (0.0, 0.0))],
            active_buf: 0,
            tab_size: (width, height),
        }
    }

    pub fn set_tab_size(&mut self, width: usize, height: usize) {
        self.tab_size = (width, height)
    }

    pub fn vertical_new(&mut self, split_direction: SplitVert) {
        let mut active_buffer = &mut self.buflist[self.active_buf];

        let pos_change_factor = (active_buffer.size.0 / 2) as f32;

        active_buffer.size.0 /= 2;

        let new_size = active_buffer.size.clone();

        let mut new_pos = active_buffer.pos.clone();

        match split_direction {
            SplitVert::Left => {
                active_buffer.pos = (active_buffer.pos.0 + pos_change_factor, active_buffer.pos.1)
            }
            SplitVert::Right => new_pos = (new_pos.0 + pos_change_factor, new_pos.1),
        }

        let new_buffer = Buffer::new(new_size, new_pos);

        self.buflist.push(new_buffer);
        self.active_buf = self.buflist.len() - 1;
    }
}
