use std::{future::Future, sync::Arc};

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

    async fn run(
        &mut self,
        frame_buffer: Arc<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>,
        clk_freq: Arc<Option<f64>>,
    ) -> impl Future<Output = Result<(), Chip8Error>> + Send;
}
