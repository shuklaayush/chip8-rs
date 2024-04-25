use std::sync::{Arc, RwLock};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
    rwlock::CheckedRead,
    util::run_loop,
};

pub trait DisplayDriver: Send {
    fn frequency(&self) -> u64;

    fn draw(
        &mut self,
        frame_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        cpu_freq: Option<f64>,
        fps: Option<f64>,
    ) -> Result<(), Chip8Error>;

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
        cpu_freq: Arc<RwLock<Option<f64>>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |elapsed| {
            // TODO: Put behind feature flag
            let fps = 1.0 / elapsed.as_secs_f64();

            self.draw(
                *frame_buffer.checked_read()?,
                *cpu_freq.checked_read()?,
                Some(fps),
            )
        });
    }
}
