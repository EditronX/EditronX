use crate::enums::LineNumber;

pub struct Settings {
    pub line_number: LineNumber,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            line_number: LineNumber::Relative,
        }
    }
}
