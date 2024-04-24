use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Paragraph},
    Terminal,
};
use std::time::{Duration, SystemTime};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
};

pub trait DisplayDriver {
    fn refresh_rate(&self) -> u64;

    fn draw(
        &mut self,
        frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    ) -> Result<(), Chip8Error>;
}

pub struct TerminalDisplay<B: Backend> {
    terminal: Terminal<B>,
    refresh_rate: u64,
    prev_time: SystemTime,
}

impl<B: Backend> TerminalDisplay<B> {
    pub fn new(terminal: Terminal<B>, refresh_rate: u64) -> Self {
        Self {
            terminal,
            refresh_rate,
            prev_time: SystemTime::now(),
        }
    }
}

impl<B: Backend> DisplayDriver for TerminalDisplay<B> {
    fn refresh_rate(&self) -> u64 {
        self.refresh_rate
    }

    fn draw(
        &mut self,
        frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    ) -> Result<(), Chip8Error> {
        let frame_interval = Duration::from_millis(1000 / self.refresh_rate());

        let frame_str = frame_buffer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "██" } else { "  " })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let block = Block::bordered();
        let area = Rect::new(
            0,
            0,
            2 * DISPLAY_WIDTH as u16 + 2,
            DISPLAY_HEIGHT as u16 + 2,
        );

        let curr_time = SystemTime::now();
        let elapsed = curr_time.duration_since(self.prev_time).unwrap_or_default();
        if elapsed >= frame_interval {
            // TODO: Put behind feature flag
            let fps = 1.0 / elapsed.as_secs_f64();
            let block = block.title(format!("FPS: {fps:.02}"));
            self.terminal
                .draw(|frame| {
                    frame.render_widget(Paragraph::new(frame_str).block(block), area);
                })
                .map_err(|e| Chip8Error::DisplayError(e.to_string()))?;

            self.prev_time = curr_time;
        }

        Ok(())
    }
}
