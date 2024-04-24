use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

use crate::{
    constants::{KEYMAP_HEX, NUM_KEYS},
    error::Chip8Error,
};

const KEYMAP: [char; NUM_KEYS] = [
    '1', '2', '3', '4', // 1 2 3 C
    'q', 'w', 'e', 'r', // 4 5 6 D
    'a', 's', 'd', 'f', // 7 8 9 E
    'z', 'x', 'c', 'v', // A 0 B F
];

pub trait InputDriver {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], Chip8Error>;
}

#[derive(Default)]
pub struct TerminalKeyboardInput {
    keys: [bool; NUM_KEYS],
}

impl InputDriver for TerminalKeyboardInput {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], Chip8Error> {
        if poll(Duration::from_micros(1)).map_err(|e| Chip8Error::KeypadError(e.to_string()))? {
            let event = read().map_err(|e| Chip8Error::KeypadError(e.to_string()))?;
            if let Event::Key(KeyEvent {
                code,
                kind,
                modifiers,
                ..
            }) = event
            {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Err(Chip8Error::Interrupt)
                    }
                    (KeyCode::Esc, _) => return Err(Chip8Error::Interrupt),
                    (KeyCode::Char(c), _) => {
                        if let Some(idx) = KEYMAP.into_iter().position(|x| x == c) {
                            self.keys[KEYMAP_HEX[idx]] = kind == KeyEventKind::Press;
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(self.keys)
    }
}
