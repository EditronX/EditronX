use std::io::Result;

use crate::{app::App, enums::Mode, functions::*};

pub fn handle_normal_mode(app: &mut App, close: &mut bool) -> Result<()> {
    let current_tab = &mut app.tabs[app.active_index];
    let current_buf = &mut current_tab.buflist[current_tab.active_buf];

    match app.key_event.code {
        crossterm::event::KeyCode::Tab => next_tab("", close, app),
        crossterm::event::KeyCode::BackTab => prev_tab("", close, app),

        crossterm::event::KeyCode::Char(ch) => match ch {
            ':' => {
                app.command = String::from(":");
                app.mode = Mode::Command;
                app.command_action = crate::enums::CommandAction::Command;
                app.error.clear();
                app.info.clear();
            }
            'i' => app.mode = Mode::Insert,
            'h' => current_buf.move_cursor(crate::enums::MoveDirection::Left, 1),
            'k' => current_buf.move_cursor(crate::enums::MoveDirection::Up, 1),
            'j' => current_buf.move_cursor(crate::enums::MoveDirection::Down, 1),
            'l' => current_buf.move_cursor(crate::enums::MoveDirection::Right, 1),
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
