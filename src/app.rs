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
    pub cursor: (usize, usize),
    pub editor_size: (usize, usize),

    pub settings: Settings,
}

impl App {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::new()],

            mode: Mode::Normal,

            command: String::new(),
            error: String::new(),
            info: String::new(),

            clipboard: Vec::new(),

            buf: String::new(),

            command_action: CommandAction::Command,
            active_index: 0,

            key_event: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            cursor: (0, 0),
            editor_size: (0, 0),

            settings: Settings::new(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| ui(self, f))?;

            let mut close = false;

            if let Event::Key(key_event) = event::read()? {
                self.key_event = key_event;

                if let KeyCode::Char(c) = key_event.code {
                    if c == 'a' {
                        self.tabs[self.active_index].vertical_new(self.settings.split_direction.0)
                    }
                }

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
