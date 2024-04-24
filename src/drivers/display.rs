use ratatui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
};

pub trait DisplayDriver {
    fn fps(&self) -> u64;

    fn draw(
        &mut self,
        frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    ) -> Result<(), Chip8Error>;
}

pub struct TerminalDisplay<B: Backend> {
    terminal: Terminal<B>,
    fps: u64,
}

impl<B: Backend> TerminalDisplay<B> {
    pub fn new(terminal: Terminal<B>, fps: u64) -> Self {
        Self { terminal, fps }
    }
}

impl<B: Backend> DisplayDriver for TerminalDisplay<B> {
    fn fps(&self) -> u64 {
        self.fps
    }

    fn draw(
        &mut self,
        frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
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

        let block = Block::bordered();
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
}
