use crate::{app::App, enums::Element, functions::*, parser::key_sequence_parser};
use std::io::Result;

pub fn handle_normal_mode(app: &mut App, close: &mut bool) -> Result<()> {
    let key_sequence_parsed = key_sequence_parser(&app.key_sequence);

    if !key_sequence_parsed.is_empty() {
        match key_sequence_parsed[0] {
            _ => {}
        }

        match key_sequence_parsed[key_sequence_parsed.len() - 1] {
            Element::Key(key_event) => match key_event.code {
                crossterm::event::KeyCode::Tab => next_tab("", close, app),
                crossterm::event::KeyCode::BackTab => prev_tab("", close, app),
                crossterm::event::KeyCode::Char(ch) => match ch {
                    'i' => go_to_insert_mode_i("", close, app),
                    'a' => go_to_insert_mode_a("", close, app),
                    ':' => go_to_command_mode("", close, app),
                    'h' => {
                        let mut count = 1;
                        if key_sequence_parsed.len() >= 2 {
                            count = match key_sequence_parsed[key_sequence_parsed.len() - 2] {
                                Element::Number(num) => num,
                                _ => 1,
                            };
                        }
                        for _ in 0..count {
                            nav_h("", close, app)
                        }
                    }
                    'j' => {
                        let mut count = 1;
                        if key_sequence_parsed.len() >= 2 {
                            count = match key_sequence_parsed[key_sequence_parsed.len() - 2] {
                                Element::Number(num) => num,
                                _ => 1,
                            };
                        }
                        for _ in 0..count {
                            nav_j("", close, app)
                        }
                    }
                    'k' => {
                        let mut count = 1;
                        if key_sequence_parsed.len() >= 2 {
                            count = match key_sequence_parsed[key_sequence_parsed.len() - 2] {
                                Element::Number(num) => num,
                                _ => 1,
                            };
                        }
                        for _ in 0..count {
                            nav_k("", close, app)
                        }
                    }
                    'l' => {
                        let mut count = 1;
                        if key_sequence_parsed.len() >= 2 {
                            count = match key_sequence_parsed[key_sequence_parsed.len() - 2] {
                                Element::Number(num) => num,
                                _ => 1,
                            };
                        }
                        for _ in 0..count {
                            nav_l("", close, app)
                        }
                    }

                    _ => {}
                },
                _ => {}
            },

            _ => {}
        }
    }

    Ok(())
}
