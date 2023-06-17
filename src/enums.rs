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

#[derive(Clone, Copy)]
pub enum LineNumber {
    Relative,
    Absolute,
    None,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum SplitVert {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum SplitHorz {
    Top,
    Bottom,
}
