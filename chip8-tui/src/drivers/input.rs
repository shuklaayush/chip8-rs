use chip8_core::{
    constants::NUM_KEYS,
    drivers::{InputDriver, InputKind},
    error::Chip8Error,
};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

const KEYMAP: [char; NUM_KEYS] = [
    '1', '2', '3', '4', // 1 2 3 C
    'q', 'w', 'e', 'r', // 4 5 6 D
    'a', 's', 'd', 'f', // 7 8 9 E
    'z', 'x', 'c', 'v', // A 0 B F
];
const FREQUENCY: u64 = 120;

#[derive(Default)]
pub struct TerminalKeyboardInput {}

impl InputDriver for TerminalKeyboardInput {
    fn frequency(&self) -> u64 {
        FREQUENCY
    }

    fn poll(&mut self) -> Result<Option<(usize, InputKind)>, Chip8Error> {
        let event = read().map_err(|e| Chip8Error::InputError(e.to_string()))?;
        if let Event::Key(KeyEvent {
            code,
            kind,
            modifiers,
            ..
        }) = event
        {
            match (modifiers, code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Err(Chip8Error::Interrupt),
                (_, KeyCode::Esc) => return Err(Chip8Error::Interrupt),
                (_, KeyCode::Char(c)) => {
                    let kind = match kind {
                        KeyEventKind::Press => Some(InputKind::Press),
                        KeyEventKind::Release => Some(InputKind::Release),
                        _ => None,
                    };

                    if let Some(kind) = kind {
                        if let Some(idx) = KEYMAP.into_iter().position(|x| x == c) {
                            return Ok(Some((idx, kind)));
                        }
                    }
                }
                _ => return Ok(None),
            }
        }

        Ok(None)
    }
}
