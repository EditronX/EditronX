use std::io::Result;

use crate::app::App;

pub fn handle_normal_mode(app: &mut App, _close: &mut bool) -> Result<()> {
    match app.key_event.code {
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
