use std::time::Duration;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::constants::NUM_KEYS;

const KEYMAP: [char; NUM_KEYS] = [
    'X', '1', '2', '3', 'Q', 'W', 'E', 'A', 'S', 'D', 'Z', 'C', '4', 'R', 'F', 'V',
];

pub trait InputDriver {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], ()>;
}

pub struct KeyboardInput {}

impl KeyboardInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl InputDriver for KeyboardInput {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], ()> {
        let mut keys = [false; NUM_KEYS];
        if poll(Duration::from_millis(1_00)).unwrap() {
            enable_raw_mode().unwrap();
            let event = read().unwrap();
            disable_raw_mode().unwrap();

            if let Event::Key(KeyEvent { code, kind, .. }) = event {
                match code {
                    KeyCode::Char(c) => {
                        let c = c.to_ascii_uppercase();
                        if let Some(idx) = KEYMAP.into_iter().position(|x| x == c) {
                            keys[idx] = kind == KeyEventKind::Press
                        }
                    }
                    KeyCode::Esc => return Err(()),
                    _ => (),
                }
            }
        }
        Ok(keys)
    }
}
