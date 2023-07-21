use crate::{app::App, enums::Mode, tab::Tab};

pub fn go_to_command_mode(_input: &str, _close: &mut bool, app: &mut App) {
    app.command = String::from(":");
    app.mode = Mode::Command;
    app.command_action = crate::enums::CommandAction::Command;
    app.error.clear();
    app.info.clear();
}

pub fn go_to_insert_mode_i(_input: &str, _close: &mut bool, app: &mut App) {
    let current_tab = &mut app.tabs[app.active_index];
    current_tab.buflist[current_tab.active_buf].move_cursor(crate::enums::MoveDirection::Right, 1);
    app.mode = Mode::Insert;
}

pub fn go_to_insert_mode_a(_input: &str, _close: &mut bool, app: &mut App) {
    app.mode = Mode::Insert;
}

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

pub fn nav_h(_input: &str, _close: &mut bool, app: &mut App) {
    let current_tab = &mut app.tabs[app.active_index];
    current_tab.buflist[current_tab.active_buf].move_cursor(crate::enums::MoveDirection::Left, 1);
}
pub fn nav_j(_input: &str, _close: &mut bool, app: &mut App) {
    let current_tab = &mut app.tabs[app.active_index];
    current_tab.buflist[current_tab.active_buf].move_cursor(crate::enums::MoveDirection::Down, 1);
}
pub fn nav_k(_input: &str, _close: &mut bool, app: &mut App) {
    let current_tab = &mut app.tabs[app.active_index];
    current_tab.buflist[current_tab.active_buf].move_cursor(crate::enums::MoveDirection::Up, 1);
}
pub fn nav_l(_input: &str, _close: &mut bool, app: &mut App) {
    let current_tab = &mut app.tabs[app.active_index];
    current_tab.buflist[current_tab.active_buf].move_cursor(crate::enums::MoveDirection::Right, 1);
}
