use std::io::Result;

use crossterm::event;

use crate::{app::App, functions::new_tab};

pub fn handle_command_mode(app: &mut App, close: &mut bool) -> Result<()> {
    match app.key_event.code {
        event::KeyCode::Char(ch) => {
            app.command.push(ch);
        }

        event::KeyCode::Backspace => {
            app.command.pop();
        }

        event::KeyCode::Esc => {
            app.mode = crate::enums::Mode::Normal;
            app.command.clear();
            app.error.clear();
            app.info.clear();
        }

        event::KeyCode::Enter => {
            let command = &app.command[1..];

            match command {
                "q" => *close = true,
                "vnew" => app.tabs[app.active_index].vertical_new(app.settings.split_direction.0),
                "tabnew" => new_tab("", close, app),
                _ => {
                    app.set_error(format!("{}: command not found", &app.command[1..]));
                }
            };

            app.mode = crate::enums::Mode::Normal;
            app.command.clear();
        }

        _ => {}
    }

    Ok(())
}
