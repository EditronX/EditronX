use crate::{
    buffer::Buffer,
    enums::{Dimensions, SplitVert},
};

pub struct Tab {
    pub buflist: Vec<Buffer>,
    pub active_index: usize,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            buflist: vec![Buffer::new(
                Dimensions::Percent(100.0, 100.0),
                Dimensions::Percent(0.0, 0.0),
            )],
            active_index: 0,
        }
    }

    pub fn vertical_new(&mut self, split_direction: SplitVert) {
        let mut active_buffer = &mut self.buflist[self.active_index];

        let pos_change_factor = match active_buffer.size {
            Dimensions::Percent(width, _height) => width as f32 / 2.0,
            Dimensions::Absolute(width, _height) => width as f32 / 2.0,
        };

        active_buffer.size = match active_buffer.size {
            Dimensions::Percent(width, height) => Dimensions::Percent(width as f32 / 2.0, height),
            Dimensions::Absolute(width, height) => Dimensions::Absolute(width / 2, height),
        };

        let new_size = active_buffer.size.clone();

        let mut new_pos = active_buffer.pos.clone();

        match split_direction {
            SplitVert::Left => {
                active_buffer.pos = match active_buffer.pos {
                    Dimensions::Percent(x, y) => Dimensions::Percent(x + pos_change_factor, y),
                    Dimensions::Absolute(x, y) => {
                        Dimensions::Absolute(x + pos_change_factor as usize, y)
                    }
                }
            }
            SplitVert::Right => {
                new_pos = match new_pos {
                    Dimensions::Absolute(x, y) => {
                        Dimensions::Absolute(x + pos_change_factor as usize, y)
                    }
                    Dimensions::Percent(x, y) => Dimensions::Percent(x + pos_change_factor, y),
                }
            }
        }

        let new_buffer = Buffer::new(new_size, new_pos);

        self.buflist.push(new_buffer);
        self.active_index = self.buflist.len() - 1;
    }
}
