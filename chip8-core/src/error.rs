use std::{error::Error, fmt::Display};

use crate::state::Address;

#[derive(Debug, Clone)]
pub enum Chip8Error {
    MemoryAccessOutOfBounds(Address),
    UnimplementedOpcode(u16),
    RomTooBig(usize),
    DisplayError(String),
    InputError(String),
    AudioError(String),
    AsyncAwaitError(String),
    MutexReadError(String),
    MutexWriteError(String),
    Interrupt,
}

impl Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip8Error::RomTooBig(size) => {
                write!(f, "ROM size too big: {size}bytes")
            }
            Chip8Error::MemoryAccessOutOfBounds(pc) => {
                write!(f, "Memory access out of bounds: 0x{:04X}", pc)
            }
            Chip8Error::UnimplementedOpcode(op) => {
                write!(f, "Unimplemented opcode: 0x{:04X}", op)
            }
            Chip8Error::DisplayError(str) => {
                write!(f, "Display Error: {str}")
            }
            Chip8Error::InputError(str) => {
                write!(f, "Input Error: {str}")
            }
            Chip8Error::AudioError(str) => {
                write!(f, "Audio Error: {str}")
            }
            Chip8Error::AsyncAwaitError(str) => {
                write!(f, "Async/Await Error: {str}")
            }
            Chip8Error::MutexReadError(str) => {
                write!(f, "Mutex read error: {str}")
            }
            Chip8Error::MutexWriteError(str) => {
                write!(f, "Mutex write error: {str}")
            }
            Chip8Error::Interrupt => {
                write!(f, "Interrupted")
            }
        }
    }
}

impl Error for Chip8Error {}
