use crate::error::Chip8Error;

pub trait AudioDriver: Send {
    fn beep(&mut self) -> Result<(), Chip8Error>;
}
