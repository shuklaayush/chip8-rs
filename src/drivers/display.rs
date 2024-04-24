use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crate::constants::{CLEAR_STR, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub trait DisplayDriver {
    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}

#[derive(Default)]
pub struct TerminalDisplay {
    frame_rate: u64,
    prev_frame: String,
}

impl TerminalDisplay {
    pub fn new(frame_rate: u64) -> Self {
        Self {
            frame_rate,
            ..Default::default()
        }
    }
}

impl DisplayDriver for TerminalDisplay {
    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        let frame = frame_buffer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "██" } else { "  " })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        if frame != self.prev_frame {
            let mut stdout = stdout();
            write!(stdout, "{CLEAR_STR}{frame}").expect("Failed to write to stdout");
            stdout.flush().expect("Failed to flush to stdout");

            self.prev_frame = frame;
        }

        sleep(Duration::from_millis(1000 / self.frame_rate))
    }
}
