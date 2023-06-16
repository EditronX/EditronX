use crate::editor::Editor;

pub enum CommandAction {
    Command,
    Error,
    Info,
}

pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
    VisualLine,
    VisualBlock,
}

pub enum BufferType {
    Editor(Editor),
    Buffer,
}

pub enum LineNumber {
    Relative,
    Absolute,
    None,
}

pub enum ShowTab {
    Always,
    Multiple,
    Never,
}

#[derive(Clone, Copy)]
pub enum Size {
    Absolute(usize, usize),
    Percent(usize, usize),
}

pub enum SplitVert {
    Left,
    Right,
}

pub enum SplitHorz {
    Top,
    Bottom,
}
