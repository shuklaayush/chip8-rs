use std::sync::{Arc, RwLock};

use crate::{error::Chip8Error, util::run_loop};

pub trait InterruptDriver: Send {
    fn check_interrupt(&mut self) -> Result<(), Chip8Error>;

    fn run(&mut self, status: Arc<RwLock<Result<(), Chip8Error>>>) {
        run_loop(status.clone(), move |_| self.check_interrupt());
    }
}
