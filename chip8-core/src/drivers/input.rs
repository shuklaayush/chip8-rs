use std::sync::{Arc, RwLock};

use crate::{
    constants::NUM_KEYS, error::Chip8Error, keypad::Key, rwlock::CheckedWrite, util::run_loop,
};

#[derive(PartialEq, Eq)]
pub enum InputKind {
    Press,
    Release,
}

pub trait InputDriver: Send {
    fn frequency(&self) -> u64;

    fn poll(&mut self) -> Result<Option<(Key, InputKind)>, Chip8Error>;

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            if let Some((key, kind)) = self.poll()? {
                // TODO: Use some kind of queue to buffer inputs
                *keypad[key as usize].checked_write()? = kind == InputKind::Press
            }
            Ok(())
        });
    }
}
