use chip8_core::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    drivers::DisplayDriver,
    error::Chip8Error,
};
use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Paragraph},
    Terminal,
};
use std::{
    future::Future,
    sync::Arc,
    time::{Duration, SystemTime},
};

pub struct TerminalDisplay<B: Backend> {
    terminal: Terminal<B>,
    refresh_rate: u64,
}

impl<B: Backend> TerminalDisplay<B> {
    pub fn new(terminal: Terminal<B>, refresh_rate: u64) -> Self {
        Self {
            terminal,
            refresh_rate,
        }
    }
}

impl<B: Backend + Send> DisplayDriver for TerminalDisplay<B> {
    fn refresh_rate(&self) -> u64 {
        self.refresh_rate
    }

    fn draw(
        &mut self,
        frame_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        clk_freq: Option<f64>,
        fps: Option<f64>,
    ) -> Result<(), Chip8Error> {
        let frame_str = frame_buffer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "██" } else { "  " })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let block = Block::bordered().title(format!(
            "{}{}",
            clk_freq.map_or("".to_string(), |f| format!("CPU: {f:.2}Hz")),
            fps.map_or("".to_string(), |f| format!(" FPS: {f:.2}Hz"))
        ));
        let area = Rect::new(
            0,
            0,
            2 * DISPLAY_WIDTH as u16 + 2,
            DISPLAY_HEIGHT as u16 + 2,
        );

        self.terminal
            .draw(|frame| {
                frame.render_widget(Paragraph::new(frame_str).block(block), area);
            })
            .map_err(|e| Chip8Error::DisplayError(e.to_string()))?;

        Ok(())
    }

    async fn run(
        &mut self,
        frame_buffer: Arc<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>,
        clk_freq: Arc<Option<f64>>,
    ) -> impl Future<Output = Result<(), Chip8Error>> + Send {
        let frame_interval = Duration::from_millis(1000 / self.refresh_rate());

        let mut prev_time = SystemTime::now();
        loop {
            let curr_time = SystemTime::now();
            let elapsed = curr_time.duration_since(prev_time).unwrap_or_default();
            if elapsed >= frame_interval {
                // TODO: Put behind feature flag
                let fps = 1.0 / elapsed.as_secs_f64();
                self.draw(*frame_buffer, *clk_freq, Some(fps));

                prev_time = curr_time;
            }
        }
    }
}
