use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

pub fn ui<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let block = Block::default().borders(Borders::ALL).title("Block");
    frame.render_widget(block, chunks[0]);
}
