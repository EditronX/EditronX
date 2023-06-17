use tui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, enums::Size};

pub fn editor_ui(app: &App) -> Vec<(List, Rect)> {
    let tab = &app.tabs[app.active_index];

    let mut list: Vec<(List, Rect)> = vec![];

    for buffer in tab.buflist.iter() {
        let rows = vec![ListItem::new("Row1"), ListItem::new("Row2")];
        let rows_list = List::new(rows).block(Block::default().borders(Borders::ALL));

        let size = match buffer.size {
            Size::Percent(x, y) => {
                let width = x / 100 * app.editor_size.0;
                let height = y / 100 * app.editor_size.1;

                (width, height)
            }
            Size::Absolute(x, y) => (x, y),
        };

        let rect = Rect::new(
            buffer.pos.0 as u16,
            if app.settings.borders.0 {
                (buffer.pos.1 + 3) as u16
            } else {
                (buffer.pos.1 + 1) as u16
            },
            size.0 as u16,
            size.1 as u16,
            // 10,
            // 10,
        );

        list.push((rows_list, rect));
    }

    // let block1 = Block::default().title("Block 1").borders(Borders::ALL);
    // let block2 = Block::default().title("Block 2").borders(Borders::ALL);
    //
    // let rect1 = Rect::new(0, 3, app.editor_size.0 as u16 / 2, app.editor_size.1 as u16);
    //
    // let rect2 = Rect::new(
    //     app.editor_size.0 as u16 / 2 + 1,
    //     3,
    //     app.editor_size.0 as u16 / 2,
    //     app.editor_size.1 as u16,
    // );
    //
    // list.push((block1, rect1));
    // list.push((block2, rect2));

    list
}
