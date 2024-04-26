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
    fn frequency(&self) -> u64 {
        self.refresh_rate
    }

    fn draw(
        &mut self,
        frame_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        cpu_freq: Option<f64>,
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
            "CHIP-8 {}{}",
            cpu_freq.map_or("".to_string(), |f| format!("{f:.2}Hz")),
            fps.map_or("".to_string(), |f| format!(" {f:.2}FPS"))
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
}
