use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;

pub fn handle_insert_mode(app: &mut App, close: &mut bool) -> Result<()> {
    let current_tab = &mut app.tabs[app.active_index];
    let current_buf = &mut current_tab.buflist[current_tab.active_buf];

    match app.key_event {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            ..
        } => current_buf.add_text_in_row(c.to_string()),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } => current_buf.new_line(true),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => app.mode = crate::enums::Mode::Normal,
        _ => {}
    }

    Ok(())
}
