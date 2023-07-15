use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use tui::{backend::Backend, Terminal};

use crate::{
    enums::{CommandAction, Mode},
    mode_handlers::{
        command::handle_command_mode, insert::handle_insert_mode, normal::handle_normal_mode,
        visual::handle_visual_mode, visual_block::handle_visual_block_mode,
        visual_line::handle_visual_line_mode,
    },
    settings::Settings,
    tab::Tab,
    ui::ui::ui,
};

pub struct App {
    pub tabs: Vec<Tab>,
    pub active_index: usize,

    pub mode: Mode,

    pub command: String,
    pub error: String,
    pub info: String,

    pub clipboard: Vec<String>,

    pub buf: String,

    pub command_action: CommandAction,

    pub key_event: KeyEvent,
    editor_size: (usize, usize),

    pub settings: Settings,
}

impl App {
    pub fn new() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap_or((0, 0));
        let settings = Settings::new();

        let tab_height = match settings.show_tabs {
            crate::enums::ShowTab::Never => height - 1,
            _ => height - 2,
        };

        Self {
            tabs: vec![Tab::new(width as usize, tab_height as usize)],

            mode: Mode::Normal,

            command: String::new(),
            error: String::new(),
            info: String::new(),

            clipboard: Vec::new(),

            buf: String::new(),

            command_action: CommandAction::Command,
            active_index: 0,

            key_event: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            editor_size: (width as usize, height as usize),

            settings,
        }
    }

    pub fn get_editor_size(&self) -> (usize, usize) {
        self.editor_size
    }

    pub fn set_editor_size(&mut self, width: usize, height: usize) {
        self.editor_size = (width, height);

        for tab in &mut self.tabs {
            tab.set_tab_size(width, height);
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| ui(self, f))?;

            let mut close = false;

            if let Event::Key(key_event) = event::read()? {
                self.key_event = key_event;

                match self.mode {
                    Mode::Normal => handle_normal_mode(self, &mut close)?,
                    Mode::Insert => handle_insert_mode(self, &mut close)?,
                    Mode::Command => handle_command_mode(self, &mut close)?,
                    Mode::Visual => handle_visual_mode(self, &mut close)?,
                    Mode::VisualLine => handle_visual_line_mode(self, &mut close)?,
                    Mode::VisualBlock => handle_visual_block_mode(self, &mut close)?,
                }
            }

            if close {
                break;
            }
        }

        Ok(())
    }

    pub fn set_error<T: AsRef<str>>(&mut self, param: T) {
        let error = param.as_ref();
        self.error = error.to_string();
        self.command_action = CommandAction::Error;
    }

    pub fn set_info<T: AsRef<str>>(&mut self, param: T) {
        let error = param.as_ref();
        self.error = error.to_string();
        self.command_action = CommandAction::Info;
    }
}
