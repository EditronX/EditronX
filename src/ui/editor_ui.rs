use tui::{
    layout::Rect,
    widgets::{Block, List, ListItem},
};

use crate::{app::App, enums::Dimensions};

pub fn editor_ui(app: &App) -> Vec<(List, Rect)> {
    let tab = &app.tabs[app.active_index];

    let mut list: Vec<(List, Rect)> = vec![];

    for (_i, buffer) in tab.buflist.iter().enumerate() {
        let rows: Vec<ListItem<'_>> = vec![ListItem::new("Row1"), ListItem::new("Row2")];
        let rows = List::new(rows).block(Block::default().style(tui::style::Style::default().bg(
            tui::style::Color::Rgb(_i as u8 * 10, _i as u8 * 20, _i as u8 * 50),
        )));

        let (width, height) = match buffer.size {
            Dimensions::Absolute(w, h) => (w as f32, h as f32),
            Dimensions::Percent(w, h) => (
                w * app.editor_size.0 as f32 / 100.0,
                h * app.editor_size.1 as f32 / 100.0,
            ),
        };

        let (x, y) = match buffer.pos {
            Dimensions::Absolute(x, y) => (x, y),
            Dimensions::Percent(x, y) => (
                x as usize * app.editor_size.0 / 100,
                y as usize * app.editor_size.1 / 100,
            ),
        };

        let rect = Rect::new(x as u16, 1 + y as u16, width as u16, height as u16);

        list.push((rows, rect))
    }

    list
}
