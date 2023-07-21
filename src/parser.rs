use crossterm::event::{KeyCode, KeyEvent};

use crate::enums::Element;

pub fn key_sequence_parser(input: &Vec<KeyEvent>) -> Vec<Element> {
    let mut elements_list: Vec<Element> = Vec::new();
    let mut current_number = String::new();

    for key_event in input.iter() {
        match key_event.code {
            KeyCode::Char(c) => {
                if c.is_digit(10) {
                    current_number.push(c);
                } else {
                    if !current_number.is_empty() {
                        let number = current_number.parse().unwrap();
                        elements_list.push(Element::Number(number));
                        current_number.clear();
                    }
                    elements_list.push(Element::Key(*key_event));
                }
            }

            _ => {
                if !current_number.is_empty() {
                    let number = current_number.parse().unwrap();
                    elements_list.push(Element::Number(number));
                    current_number.clear();
                }
                elements_list.push(Element::Key(*key_event));
            }
        }
    }

    if !current_number.is_empty() {
        let number = current_number.parse().unwrap();
        elements_list.push(Element::Number(number))
    }

    elements_list
}
