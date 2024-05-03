use std::sync::{Arc, RwLock};

use crate::{error::Chip8Error, rwlock::CheckedRead, util::run_loop_at_freq};

pub trait AudioDriver: Send {
    fn frequency(&self) -> u64;

    fn beep(&mut self) -> Result<(), Chip8Error>;

    fn run(&mut self, status: Arc<RwLock<Result<(), Chip8Error>>>, sound_timer: Arc<RwLock<u8>>) {
        run_loop_at_freq(status.clone(), self.frequency(), move |_| {
            if *sound_timer.checked_read()? > 0 {
                self.beep()?;
            }
            Ok(())
        });
    }
}
