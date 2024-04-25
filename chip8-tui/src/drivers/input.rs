use chip8_core::{
    constants::{KEYMAP_HEX, NUM_KEYS},
    drivers::InputDriver,
    error::Chip8Error,
};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::sync::{Arc, RwLock};

const KEYMAP: [char; NUM_KEYS] = [
    '1', '2', '3', '4', // 1 2 3 C
    'q', 'w', 'e', 'r', // 4 5 6 D
    'a', 's', 'd', 'f', // 7 8 9 E
    'z', 'x', 'c', 'v', // A 0 B F
];

#[derive(Default)]
pub struct TerminalKeyboardInput {}

impl InputDriver for TerminalKeyboardInput {
    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    ) -> Result<(), Chip8Error> {
        while status.read().unwrap().is_ok() {
            let event = read().map_err(|e| Chip8Error::KeypadError(e.to_string()))?;
            if let Event::Key(KeyEvent {
                code,
                kind,
                modifiers,
                ..
            }) = event
            {
                match (modifiers, code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        return Err(Chip8Error::Interrupt)
                    }
                    (_, KeyCode::Esc) => return Err(Chip8Error::Interrupt),
                    (_, KeyCode::Char(c)) => {
                        if let Some(idx) = KEYMAP.into_iter().position(|x| x == c) {
                            *keypad[KEYMAP_HEX[idx]].write().unwrap() = kind == KeyEventKind::Press;
                        }
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    }
}
