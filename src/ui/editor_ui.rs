use tui::{
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{List, ListItem},
};

use crate::app::App;

pub fn editor_ui(app: &App) -> Vec<(List, Rect)> {
    let tab = &app.tabs[app.active_index];

    let mut list: Vec<(List, Rect)> = vec![];

    for (_i, buffer) in tab.buflist.iter().enumerate() {
        let rows: Vec<_> = buffer
            .get_rows()
            .iter()
            .map(|cells| {
                let cells: Vec<Span> = cells
                    .iter()
                    .map(|cell| {
                        Span::styled(
                            &cell.symbol,
                            Style::default()
                                .fg(cell.fg)
                                .bg(cell.bg)
                                .add_modifier(cell.modifier),
                        )
                    })
                    .collect();

                ListItem::new(Spans::from(cells))
            })
            .collect();

        let rows = List::new(rows);

        let (width, height) = buffer.size;
        let (x, y) = buffer.pos;

        let rect = Rect::new(
            x as u16,
            match app.settings.show_tabs {
                crate::enums::ShowTab::Never => 0,
                crate::enums::ShowTab::Always => 1,
                crate::enums::ShowTab::Multiple => {
                    if app.tabs.len() > 1 {
                        1
                    } else {
                        0
                    }
                }
            } + y as u16,
            width as u16,
            height as u16,
        );

        list.push((rows, rect))
    }

    list
}
