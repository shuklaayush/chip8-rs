use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{
    error::Chip8Error,
    input::{InputEvent, InputQueue},
    rwlock::{CheckedRead, CheckedWrite},
    util::run_loop,
};

pub trait InputDriver: Send {
    fn frequency(&self) -> u64;

    fn poll(&mut self) -> Result<Option<InputEvent>, Chip8Error>;

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        queue: Arc<RwLock<VecDeque<(InputEvent, u64)>>>,
        clk: Arc<RwLock<u64>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            if let Some(event) = self.poll()? {
                let clk = *clk.checked_read()?;
                (*queue.checked_write()?).enqueue(event, clk);
            }
            Ok(())
        });
    }
}
