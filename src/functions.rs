use crate::{app::App, tab::Tab};

pub fn new_tab(_input: &str, _close: &mut bool, app: &mut App) {
    let (width, height) = app.get_editor_size();
    app.tabs
        .insert(app.active_index + 1, Tab::new(width, height));
    app.active_index += 1;
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
