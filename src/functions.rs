use crate::{
    app::App,
    enums::{Element, Mode},
    parser::key_sequence_parser,
    tab::Tab,
};

pub fn go_to_command_mode(_input: &str, _close: &mut bool, app: &mut App) {
    app.command = String::from(":");
    app.mode = Mode::Command;
    app.command_action = crate::enums::CommandAction::Command;
    app.error.clear();
    app.info.clear();

    app.key_sequence.clear();
}

pub fn go_to_insert_mode_i(_input: &str, _close: &mut bool, app: &mut App) {
    let current_tab = &mut app.tabs[app.active_index];
    current_tab.buflist[current_tab.active_buf].move_cursor(crate::enums::MoveDirection::Right, 1);
    app.mode = Mode::Insert;

    app.key_sequence.clear();
}

pub fn go_to_insert_mode_a(_input: &str, _close: &mut bool, app: &mut App) {
    app.mode = Mode::Insert;

    app.key_sequence.clear();
}

pub fn new_tab(_input: &str, _close: &mut bool, app: &mut App) {
    let (width, height) = app.get_editor_size();
    app.tabs
        .insert(app.active_index + 1, Tab::new(width, height));
    app.active_index += 1;

    app.key_sequence.clear();
}

pub fn next_tab(_input: &str, _close: &mut bool, app: &mut App) {
    app.active_index = (app.active_index + 1) % app.tabs.len();

    app.key_sequence.clear();
}

pub fn prev_tab(_input: &str, _close: &mut bool, app: &mut App) {
    app.active_index = if app.active_index == 0 {
        app.tabs.len() - 1
    } else {
        app.active_index - 1
    };

    app.key_sequence.clear();
}

pub fn nav(_input: &str, _close: &mut bool, app: &mut App) {
    let key_sequence_parsed = key_sequence_parser(&app.key_sequence);

    let current_tab = &mut app.tabs[app.active_index];

    if !key_sequence_parsed.is_empty() {
        let direction_char = match key_sequence_parsed[key_sequence_parsed.len() - 1] {
            Element::Key(key_event) => match key_event.code {
                crossterm::event::KeyCode::Char(c) => c,
                _ => ' ',
            },
            _ => ' ',
        };

        let direction = match direction_char {
            'h' => crate::enums::MoveDirection::Left,
            'j' => crate::enums::MoveDirection::Down,
            'k' => crate::enums::MoveDirection::Up,
            'l' => crate::enums::MoveDirection::Right,
            _ => unreachable!(),
        };

        let mut count = 1;
        if key_sequence_parsed.len() >= 2 {
            count = match key_sequence_parsed[key_sequence_parsed.len() - 2] {
                Element::Number(num) => num,
                _ => 1,
            };
        }
        current_tab.buflist[current_tab.active_buf].move_cursor(direction, count);
    }

    app.key_sequence.clear();
}
