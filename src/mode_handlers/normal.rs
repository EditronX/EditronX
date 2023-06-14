use std::io::Result;

use crate::app::App;

pub fn handle_normal_mode(app: &mut App, close: &mut bool) -> Result<()> {
    match app.key_event.code {
        crossterm::event::KeyCode::Char(ch) => {
            if ch == 'q' {
                *close = true
            }
        }
        _ => {}
    }

    Ok(())
}
