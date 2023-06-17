use crate::{
    buffer::Buffer,
    enums::{Size, SplitVert},
};

pub struct Tab {
    pub buflist: Vec<Buffer>,
    pub active_index: usize,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            buflist: vec![Buffer::new(Size::Percent(100, 100), (0, 0))],
            active_index: 0,
        }
    }

    pub fn vertical_new(&mut self, split_direction: SplitVert) {
        let prev_buf = &mut self.buflist[self.active_index];

        let mut x = 0;
        let _x = x; // just to remove the annoying warning that overwritten before use
        let y = prev_buf.pos.1;

        let term_size = crossterm::terminal::size().expect("Failed to get terminal size");

        let new_size = match prev_buf.size {
            Size::Absolute(w, h) => match split_direction {
                SplitVert::Left => {
                    x = prev_buf.pos.0;
                    prev_buf.pos.0 += w / 2;

                    Size::Absolute(w / 2, h)
                }
                SplitVert::Right => {
                    x = prev_buf.pos.0 + (w / 2);

                    Size::Absolute(w / 2, h)
                }
            },
            Size::Percent(w, h) => match split_direction {
                SplitVert::Left => {
                    x = prev_buf.pos.0;
                    prev_buf.pos.0 += w / 200 * term_size.0 as usize;

                    Size::Percent(w / 2, h)
                }
                SplitVert::Right => {
                    x = prev_buf.pos.0 + (w / 200 * term_size.0 as usize);

                    Size::Percent(w / 2, h)
                }
            },
        };

        prev_buf.size = new_size.clone();

        let buffer = Buffer::new(new_size, (x, y));
        self.buflist.push(buffer);
        self.active_index = self.buflist.len() - 1;
    }
}
