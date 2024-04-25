use std::sync::{Arc, RwLock};

use crate::{constants::NUM_KEYS, error::Chip8Error};

pub trait InputDriver: Send {
    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    ) -> Result<(), Chip8Error>;
}
