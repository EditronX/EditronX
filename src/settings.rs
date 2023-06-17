use crate::enums::{LineNumber, ShowTab, SplitHorz, SplitVert};

#[derive(Clone, Copy)]
pub struct Settings {
    pub line_number: LineNumber,
    pub borders: (bool, bool), // tabs, editor
    pub show_tab_title: bool,
    pub show_tabs: ShowTab,
    pub tab_numbering: bool,
    pub split_direction: (SplitVert, SplitHorz),
}

impl Settings {
    pub fn new() -> Self {
        Self {
            line_number: LineNumber::Relative,
            borders: (true, true),
            show_tab_title: true,
            show_tabs: ShowTab::Always,
            tab_numbering: true,
            split_direction: (SplitVert::Right, SplitHorz::Bottom),
        }
    }
}
