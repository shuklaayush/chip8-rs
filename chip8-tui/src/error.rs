use chip8_core::error::Chip8Error;
use std::{error::Error, fmt::Display};

// TODO: Use eyre for errors
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum TuiError {
    RomReadError(String),
    TerminalSetupError(String),
    TerminalRestoreError(String),
    Chip8Error(Chip8Error),
    InputError(String),
}

impl Display for TuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TuiError::RomReadError(str) => {
                write!(f, "Unable to read ROM: {str}")
            }
            TuiError::TerminalSetupError(str) => {
                write!(f, "Unable to setup terminal: {str}")
            }
            TuiError::TerminalRestoreError(str) => {
                write!(f, "Unable to restore terminal: {str}")
            }
            TuiError::Chip8Error(e) => {
                write!(f, "{e}")
            }
            TuiError::InputError(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl Error for TuiError {}
