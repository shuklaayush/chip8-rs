use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Chip8Error {
    ProgramCounterOverflow(u16),
    UnimplementedOpcode(u16),
    RomTooBig(usize),
    DisplayError(String),
    KeypadError(String),
    AudioError(String),
    Interrupt,
}

impl Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip8Error::RomTooBig(size) => {
                write!(f, "ROM size too big: {size}bytes")
            }
            Chip8Error::ProgramCounterOverflow(pc) => {
                write!(f, "Program counter overflow: 0x{:04X}", pc)
            }
            Chip8Error::UnimplementedOpcode(op) => {
                write!(f, "Unimplemented opcode: 0x{:04X}", op)
            }
            Chip8Error::DisplayError(str) => {
                write!(f, "Display Error: {str}")
            }
            Chip8Error::KeypadError(str) => {
                write!(f, "Keypad Error: {str}")
            }
            Chip8Error::AudioError(str) => {
                write!(f, "Audio Error: {str}")
            }
            _ => Ok(()),
        }
    }
}
