use tui::{buffer::Cell, style::Modifier};

use crate::enums::{BufferType, MoveDirection};

static mut CURSOR_X_POS: usize = 0;

pub struct Buffer {
    pub size: (usize, usize),
    pub is_float: bool,
    rows: Vec<Vec<Cell>>,
    pub title: String,
    cursor: (usize, usize),
    pub offset: (usize, usize),
    pub buffer_type: BufferType,
    pub hidden: bool,
    pub pos: (f32, f32),
}

impl Buffer {
    pub fn new(size: (usize, usize), pos: (f32, f32)) -> Buffer {
        Buffer {
            size,
            pos,
            is_float: false,
            rows: vec![vec![]],
            title: String::from("[No Name]"),
            cursor: (0, 0),
            offset: (0, 0),
            buffer_type: BufferType::Buffer,
            hidden: false,
        }
    }

    // getters
    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn get_rows(&self) -> &Vec<Vec<Cell>> {
        &self.rows
    }

    fn string_to_cells(text: String) -> Vec<Cell> {
        let mut cells = vec![];

        for c in text.chars() {
            cells.push(Cell {
                fg: tui::style::Color::Reset,
                bg: tui::style::Color::Reset,
                symbol: c.to_string(),
                modifier: Modifier::DIM,
            });
        }

        cells
    }

    pub fn new_line(&mut self, below: bool) {
        if below {
            self.cursor.1 += 1;
            self.rows.insert(self.cursor.1, vec![]);
        } else {
            self.cursor.1 -= if self.cursor.1 > 0 { 1 } else { 0 };
        }
        self.cursor.0 = 0;
    }

    pub fn add_text_in_row(&mut self, text: String) {
        let cursor_x = &mut self.cursor.0;
        let cursor_y = &self.cursor.1;

        let offset_x = &mut self.offset.0;
        let offset_y = &self.offset.1;

        if self.rows.len() == 0 {
            self.rows.push(Self::string_to_cells(text.clone()));
        } else {
            let current_row = &mut self.rows[*cursor_y + *offset_y];
            let current_pos = *cursor_x + *offset_x;

            current_row.splice(current_pos..current_pos, Self::string_to_cells(text));
        }

        *cursor_x += 1;
    }

    pub fn add_text(&mut self, text: String) {
        for ch in text.chars() {
            if ch == '\n' {
                self.new_line(true);
            } else {
                self.add_text_in_row(ch.to_string());
            }
        }
    }

    pub fn move_cursor(&mut self, direction: MoveDirection, steps: usize) {
        let cursor_x = &mut self.cursor.0;
        let cursor_y = &mut self.cursor.1;

        let offset_x = &mut self.offset.0;
        let offset_y = &mut self.offset.1;

        match direction {
            MoveDirection::Up => {
                if *cursor_y >= steps {
                    *cursor_y -= steps;
                } else {
                    *offset_y = offset_y.saturating_sub(steps - *cursor_y);
                    *cursor_y = 0
                }

                if self.rows[*cursor_y + *offset_y].len() < *cursor_x {
                    *cursor_x = self.rows[*cursor_y + *offset_y].len();
                }
            }
            MoveDirection::Down => {
                if *cursor_y + *offset_y + steps <= self.rows.len() {
                    if *cursor_y + steps >= self.size.1 {
                        // *offset_y += steps - (self.size.1 - *cursor_y);
                        // *cursor_y = self.size.1;
                    } else {
                        *cursor_y += steps;
                    }
                }
            }

            // MoveDirection::Right => {
            //     *
            // }

            // MoveDirection::Left => {
            //     *cursor_x -= if *cursor_x > 0 { 1 } else { 0 };
            //     if *cursor_x < *offset_x {
            //         *offset_x -= if *offset_x > 0 { 1 } else { 0 };
            //     }
            // }
            // MoveDirection::Right => {
            //     *cursor_x += if *cursor_x < self.rows[*cursor_y].len() - 1 {
            //         1
            //     } else {
            //         0
            //     };
            //     if *cursor_x >= self.size.width + *offset_x {
            //         *offset_x += 1;
            //     }
            // }
            _ => {}
        }
    }
}
