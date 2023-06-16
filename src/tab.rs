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
        let mut buffer = Buffer::new();
        buffer.set_size(Size::Percent(100, 100));

        Self {
            buflist: vec![Buffer::new()],
            active_index: 0,
        }
    }

    pub fn vertical_new(&mut self, split_direction: SplitVert) {
        let mut buffer = Buffer::new();

        let prev_buf = &mut self.buflist[self.active_index];

        let mut x = 0;
        let y = prev_buf.pos.1;

        let new_size = match prev_buf.size {
            Size::Absolute(w, h) => match split_direction {
                SplitVert::Left => {
                    x = prev_buf.pos.0;
                    prev_buf.pos.0 += w / 2;

                    Size::Absolute(w / 2, h)
                }
                SplitVert::Right => {
                    x = prev_buf.pos.0 + (w / 2);

                    Size::Absolute(w - (w / 2), h)
                }
            },
            Size::Percent(w, h) => match split_direction {
                SplitVert::Left => Size::Percent(w / 2, h),
                SplitVert::Right => Size::Percent(w - (w / 2), h),
            },
        };

        prev_buf.size = new_size.clone();

        buffer.set_size(new_size);

        buffer.set_pos(x, y);
    }
}
