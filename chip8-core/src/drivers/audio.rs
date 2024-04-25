use crate::error::Chip8Error;

pub trait AudioDriver {
    fn beep(&mut self) -> Result<(), Chip8Error>;
}
