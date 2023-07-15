use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::Tabs,
};

use crate::app::App;

pub fn tabs_ui(app: &mut App) -> Tabs {
    fn tab_title(numbering: bool, title: String, i: usize) -> String {
        if numbering {
            format!("{}. {}", i + 1, title)
        } else {
            title
        }
    }

    let tab_titles = app
        .tabs
        .iter()
        .enumerate()
        .map(|(i, _tab)| {
            Spans::from(vec![Span::styled(
                tab_title(
                    app.settings.tab_numbering,
                    app.tabs[i].buflist[app.tabs[i].active_buf].title.clone(),
                    i,
                ),
                Style::default(),
            )])
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .select(app.active_index)
        .highlight_style(Style::default().fg(tui::style::Color::Yellow));

    tabs
}
