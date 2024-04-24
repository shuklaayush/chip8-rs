use std::io::{stdout, Write};

pub trait AudioDriver {
    fn beep(&mut self);
}

#[derive(Default)]
pub struct TerminalAudio {}

impl AudioDriver for TerminalAudio {
    fn beep(&mut self) {
        let mut stdout = stdout();
        write!(stdout, "\x07").expect("Failed to write to stdout");
        stdout.flush().expect("Failed to flush to stdout");
    }
}
