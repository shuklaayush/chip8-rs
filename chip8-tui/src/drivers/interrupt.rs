use chip8_core::{drivers::InterruptDriver, error::Chip8Error};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Default)]
pub struct TerminalKeyboardInterrupt {}

impl InterruptDriver for TerminalKeyboardInterrupt {
    fn check_interrupt(&mut self) -> Result<(), Chip8Error> {
        let event = read().map_err(|e| Chip8Error::InputError(e.to_string()))?;
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match (modifiers, code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Err(Chip8Error::Interrupt),
                (_, KeyCode::Esc) => return Err(Chip8Error::Interrupt),
                _ => return Ok(()),
            }
        }

        Ok(())
    }
}
