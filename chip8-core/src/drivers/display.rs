use std::{
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
};

pub trait DisplayDriver: Send {
    fn refresh_rate(&self) -> u64;

    fn draw(
        &mut self,
        frame_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        clk_freq: Option<f64>,
        fps: Option<f64>,
    ) -> Result<(), Chip8Error>;

    fn run(
        &mut self,
        shared_res: Arc<RwLock<Result<(), Chip8Error>>>,
        frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
        clk_freq: Arc<RwLock<Option<f64>>>,
    ) -> Result<(), Chip8Error> {
        let frame_interval = Duration::from_millis(1000 / self.refresh_rate());

        let mut prev_time = SystemTime::now();
        while shared_res.read().unwrap().is_ok() {
            let curr_time = SystemTime::now();
            let elapsed = curr_time.duration_since(prev_time).unwrap_or_default();
            if elapsed >= frame_interval {
                // TODO: Put behind feature flag
                let fps = 1.0 / elapsed.as_secs_f64();

                self.draw(
                    *frame_buffer.read().unwrap(),
                    *clk_freq.read().unwrap(),
                    Some(fps),
                )?;

                prev_time = curr_time;
            }
        }

        Ok(())
    }
}
