use std::io::Result;

use crate::{app::App, functions::*};

pub fn handle_normal_mode(app: &mut App, close: &mut bool) -> Result<()> {
    match app.key_event.code {
        crossterm::event::KeyCode::Tab => next_tab("", close, app),
        crossterm::event::KeyCode::BackTab => prev_tab("", close, app),

        crossterm::event::KeyCode::Char(ch) => match ch {
            ':' => {
                app.command = String::from(":");
                app.mode = crate::enums::Mode::Command;
                app.command_action = crate::enums::CommandAction::Command;
                app.error.clear();
                app.info.clear();
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
