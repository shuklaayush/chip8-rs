use crossterm::{
    cursor::{Hide, Show},
    execute,
};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crate::constants::{CLEAR_STR, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub trait DisplayDriver {
    fn fps(&self) -> u64;
    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}

#[derive(Default)]
pub struct TerminalDisplay {
    fps: u64,
    prev_frame: String,
}

impl TerminalDisplay {
    pub fn new(fps: u64) -> Self {
        execute!(stdout(), Hide).expect("Failed to hide cursor");
        Self {
            fps,
            prev_frame: Default::default(),
        }
    }
}

impl Drop for TerminalDisplay {
    fn drop(&mut self) {
        execute!(stdout(), Show).expect("Failed to show cursor");
    }
}

impl DisplayDriver for TerminalDisplay {
    fn fps(&self) -> u64 {
        self.fps
    }

    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        let frame = frame_buffer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "██" } else { "  " })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\r\n");

        if frame != self.prev_frame {
            let mut stdout = stdout();
            write!(stdout, "{CLEAR_STR}{frame}").expect("Failed to write to stdout");
            stdout.flush().expect("Failed to flush to stdout");

            self.prev_frame = frame;
        }

        sleep(Duration::from_millis(1000 / self.fps))
    }
}
