use crate::{constants::NUM_KEYS, error::Chip8Error};

pub trait InputDriver: Send {
    fn poll(&mut self, keypad: &mut [bool; NUM_KEYS]) -> Result<(), Chip8Error>;
}
