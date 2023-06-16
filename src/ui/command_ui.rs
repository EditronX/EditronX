use tui::{
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
};

use crate::{app::App, enums::CommandAction};

pub fn command_ui(app: &mut App) -> Paragraph {
    let paragraph = match app.command_action {
        CommandAction::Command => Paragraph::new(app.command.clone()),
        CommandAction::Info => Paragraph::new(app.info.clone()),
        CommandAction::Error => Paragraph::new(Span::styled(
            app.error.clone(),
            Style::default().fg(Color::Red),
        )),
    };

    paragraph
}
