use crossterm::{
    event::{
        poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{io::stdout, time::Duration};

use crate::constants::{KEYMAP_HEX, NUM_KEYS};

const KEYMAP: [char; NUM_KEYS] = [
    '1', '2', '3', '4', // 1 2 3 C
    'q', 'w', 'e', 'r', // 4 5 6 D
    'a', 's', 'd', 'f', // 7 8 9 E
    'z', 'x', 'c', 'v', // A 0 B F
];

pub trait InputDriver {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], ()>;
}

#[derive(Default)]
pub struct TerminalKeyboardInput {
    keys: [bool; NUM_KEYS],
}

impl TerminalKeyboardInput {
    pub fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode");
        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )
        .expect("Failed to enable kitty protocol");
        Self {
            keys: Default::default(),
        }
    }
}

impl Drop for TerminalKeyboardInput {
    fn drop(&mut self) {
        execute!(stdout(), PopKeyboardEnhancementFlags).expect("Failed to disable kitty protocol");
        disable_raw_mode().expect("Failed to disable raw mode");
    }
}

impl InputDriver for TerminalKeyboardInput {
    fn poll(&mut self) -> Result<[bool; NUM_KEYS], ()> {
        if poll(Duration::from_micros(1)).expect("Failed to poll event") {
            let event = read().expect("Failed to read input");
            if let Event::Key(KeyEvent {
                code,
                kind,
                modifiers,
                ..
            }) = event
            {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Err(()),
                    (KeyCode::Esc, _) => return Err(()),
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
