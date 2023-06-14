use crate::editor::Editor;

#[derive(Debug, Clone, Copy)]
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
