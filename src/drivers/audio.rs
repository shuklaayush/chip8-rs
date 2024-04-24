use std::io::{stdout, Write};

pub trait AudioDriver {
    fn beep(&mut self);
}

#[derive(Default)]
pub struct TerminalAudio {}

impl AudioDriver for TerminalAudio {
    fn beep(&mut self) {
        let mut stdout = stdout();
        write!(stdout, "\x07").unwrap();
        stdout.flush().unwrap();
    }
}
