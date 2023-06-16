use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    app::App,
    enums::{CommandAction, Mode, ShowTab},
};

use super::{command_ui::command_ui, tabs_ui::tabs_ui};

pub fn ui<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let mut chunks_list = vec![
        Constraint::Length(if app.settings.borders.0 { 3 } else { 1 }),
        Constraint::Min(1),
        Constraint::Length(1),
    ];

    match app.settings.show_tabs {
        ShowTab::Never => {
            chunks_list.remove(0);
        }
        ShowTab::Multiple => {
            if app.tabs.len() <= 1 {
                chunks_list.remove(0);
            }
        }
        ShowTab::Always => {}
    }

    let chunks = Layout::default()
        .constraints(chunks_list.as_ref())
        .split(frame.size());
    match app.settings.show_tabs {
        ShowTab::Never => {}
        ShowTab::Multiple => {
            if app.tabs.len() > 1 {
                frame.render_widget(tabs_ui(app), chunks[0]);
            }
        }
        ShowTab::Always => {
            frame.render_widget(tabs_ui(app), chunks[0]);
        }
    }

    frame.render_widget(command_ui(app), chunks[chunks.len() - 1]);

    app.command_action = CommandAction::Command;

    // cursor stuff
    match app.mode {
        Mode::Command => {
            let cursor_position = app.command.len() as u16;
            frame.set_cursor(
                chunks[chunks.len() - 1].x + cursor_position,
                chunks[chunks.len() - 1].y + 1,
            );
        }
        _ => {}
    }
}
