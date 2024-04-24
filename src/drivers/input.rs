use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

use crate::constants::NUM_KEYS;

const KEYMAP: [char; NUM_KEYS] = [
    'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 'z', 'c', '4', 'r', 'f', 'v',
];

pub trait InputDriver {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], ()>;
}

#[derive(Default)]
pub struct KeyboardInput {}

impl InputDriver for KeyboardInput {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], ()> {
        let mut keys = [false; NUM_KEYS];
        enable_raw_mode().unwrap();
        if poll(Duration::from_millis(1_00)).unwrap() {
            let event = read().unwrap();

            if let Event::Key(KeyEvent {
                code,
                kind,
                modifiers,
                ..
            }) = event
            {
                match (code, modifiers) {
                    (KeyCode::Esc, _) => return Err(()),
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Err(()),
                    (KeyCode::Char(c), _) => {
                        if let Some(idx) = KEYMAP.into_iter().position(|x| x == c) {
                            keys[idx] = kind == KeyEventKind::Press
                        }
                    }
                    _ => (),
                }
            }
        }
        disable_raw_mode().unwrap();
        Ok(keys)
    }
}
