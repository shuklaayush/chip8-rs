use super::constants::{CLEAR_STR, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub trait DisplayDriver {
    fn draw(&mut self, frame_buffer: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}

#[derive(Default)]
pub struct TerminalDisplay {
    prev_frame: String,
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
            print!("{CLEAR_STR}{frame}");
            self.prev_frame = frame;
        }
    }
}
