use ratatui::{backend::Backend, widgets::Paragraph, Terminal};

use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub trait DisplayDriver {
    fn fps(&self) -> u64;
    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
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

    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        let frame_str = frame_buffer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "██" } else { "  " })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\r\n");

        self.terminal
            .draw(|frame| {
                frame.render_widget(Paragraph::new(frame_str), frame.size());
            })
            .expect("Failed to draw");
    }
}
