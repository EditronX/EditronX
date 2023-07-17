use std::io::stdout;

use crossterm::{cursor::SetCursorStyle, execute};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    app::App,
    enums::{Mode, ShowTab},
};

use super::{command_ui::command_ui, editor_ui::editor_ui, tabs_ui::tabs_ui};

pub fn ui<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let mut chunks_list = vec![
        Constraint::Length(1),
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

    // render tabs
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

    // set editor size
    app.set_editor_size(
        chunks[chunks_list.len() - 2].width as usize,
        chunks[chunks_list.len() - 2].height as usize,
    );

    // render editor
    for (rows_list, rect) in editor_ui(app) {
        frame.render_widget(rows_list, rect);
    }

    // render command ui
    frame.render_widget(command_ui(app), chunks[chunks.len() - 1]);

    let tab = &mut app.tabs[app.active_index];
    let buffer = &mut tab.buflist[tab.active_buf];

    // CURSOR STUFF
    match app.mode {
        Mode::Command => {
            let cursor_position = app.command.len() as u16;
            frame.set_cursor(
                chunks[chunks.len() - 1].x + cursor_position,
                chunks[chunks.len() - 1].y + 1,
            );
        }
        Mode::Normal => {
            let cursor_position = buffer.get_cursor();

            frame.set_cursor(
                chunks[chunks.len() - 2].x + cursor_position.0.saturating_sub(1) as u16,
                chunks[chunks.len() - 2].y + cursor_position.1 as u16,
            );
        }
        _ => {
            let cursor_position = buffer.get_cursor();

            frame.set_cursor(
                chunks[chunks.len() - 2].x + cursor_position.0 as u16,
                chunks[chunks.len() - 2].y + cursor_position.1 as u16,
            );
        }
    }

    // cursor style
    match app.mode {
        Mode::Insert => {
            execute!(stdout(), app.settings.insert_mode_cursor).expect("Couldn't set cursor style")
        }

        Mode::Normal => {
            execute!(stdout(), app.settings.normal_mode_cursor).expect("Couldn't set cursor style")
        }

        _ => execute!(stdout(), SetCursorStyle::SteadyBlock).expect("Couldn't set cursor style"),
    }
}
