use std::sync::{Arc, RwLock};

use crate::{
    constants::{KEYMAP_HEX, NUM_KEYS},
    error::Chip8Error,
    util::run_loop,
};

#[derive(PartialEq, Eq)]
pub enum InputKind {
    Press,
    Release,
}

pub trait InputDriver: Send {
    fn frequency(&self) -> u64;

    fn poll(&self) -> Result<Option<(usize, InputKind)>, Chip8Error>;

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    ) -> Result<(), Chip8Error> {
        run_loop(status, self.frequency(), move |_| {
            if let Some((idx, kind)) = self.poll()? {
                *keypad[KEYMAP_HEX[idx]].write().unwrap() = kind == InputKind::Press;
            }
            Ok(())
        })
    }
}
