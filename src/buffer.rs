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
            if self.cursor.1 < self.size.1 - 1 {
                self.cursor.1 += 1;
            } else {
                self.offset.1 += 1;
            }
        } else {
            self.cursor.1 -= if self.cursor.1 > 0 { 1 } else { 0 };
        }

        self.rows.insert(self.cursor.1 + self.offset.1, vec![]);

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

            current_row.splice(
                current_pos..current_pos,
                Self::string_to_cells(text.clone()),
            );
        }

        *cursor_x += text.len();

        if *cursor_x >= self.size.0 - 1 {
            *offset_x += text.len() - (self.size.0 - *cursor_x);
            *cursor_x = self.size.0 - 1;
        }

        unsafe {
            CURSOR_X_POS = *cursor_x;
        }
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

        let current_row = *cursor_y + *offset_y;

        let size_x = self.size.0;
        let size_y = self.size.1;

        match direction {
            MoveDirection::Up => {
                if *cursor_y >= steps {
                    *cursor_y -= steps;
                } else {
                    *offset_y = offset_y.saturating_sub(steps - *cursor_y);
                    *cursor_y = 0
                }

                unsafe {
                    *cursor_x = if CURSOR_X_POS > self.rows[*cursor_y + *offset_y].len() {
                        self.rows[*cursor_y + *offset_y].len()
                    } else {
                        CURSOR_X_POS
                    };
                }
            }
            MoveDirection::Down => {
                if *cursor_y + steps > size_y {
                    *offset_y += steps - (size_y - *cursor_y);
                    *cursor_y = size_y - 1;

                    if *cursor_y + *offset_y > self.rows.len() - 1 {
                        *offset_y = self.rows.len() - 1 - *cursor_y;
                    }
                } else {
                    *cursor_y += steps;

                    if *cursor_y + *offset_y > self.rows.len() - 1 {
                        *cursor_y = self.rows.len() - 1 - *offset_y;
                    }
                }

                unsafe {
                    *cursor_x = if CURSOR_X_POS > self.rows[*cursor_y + *offset_y].len() {
                        self.rows[*cursor_y + *offset_y].len()
                    } else {
                        CURSOR_X_POS
                    };
                }
            }

            MoveDirection::Left => {
                if *cursor_x >= steps {
                    *cursor_x -= steps;
                } else {
                    *offset_x = offset_x.saturating_sub(steps - *cursor_x);
                    *cursor_x = 0;
                }

                unsafe {
                    CURSOR_X_POS = *cursor_x;
                }
            }
            MoveDirection::Right => {
                if *cursor_x + steps >= size_x {
                    *offset_x += steps - (size_x - *cursor_x);
                    *cursor_x = size_x;

                    if *cursor_x + *offset_x > self.rows[current_row].len() {
                        *offset_x = self.rows[current_row].len() - *cursor_x;
                    }
                } else {
                    *cursor_x += steps;

                    if *cursor_x + *offset_x > self.rows[current_row].len() {
                        *cursor_x = self.rows[current_row].len() - *offset_x;
                    }
                }

                unsafe {
                    CURSOR_X_POS = *cursor_x;
                }
            }
        }
    }
}
