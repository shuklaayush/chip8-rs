use chip8_core::drivers::AudioDriver;

#[derive(Default)]
pub struct TerminalAudio {}

impl AudioDriver for TerminalAudio {
    fn beep(&mut self) -> Result<(), Chip8Error> {
        let mut stdout = stdout();
        write!(stdout, "\x07").map_err(|e| Chip8Error::AudioError(e.to_string()))?;
        stdout
            .flush()
            .map_err(|e| Chip8Error::AudioError(e.to_string()))
    }
}
