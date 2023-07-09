use crate::{app::App, tab::Tab};

pub fn new_tab(_input: &str, _close: &mut bool, app: &mut App) {
    app.tabs.push(Tab::new());
}

pub fn next_tab(_input: &str, _close: &mut bool, app: &mut App) {
    app.active_index = (app.active_index + 1) % app.tabs.len();
}

pub fn prev_tab(_input: &str, _close: &mut bool, app: &mut App) {
    app.active_index = if app.active_index == 0 {
        app.tabs.len() - 1
    } else {
        app.active_index - 1
    };
}
