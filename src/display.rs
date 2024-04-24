use super::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

const CLEAR_STR: &str = "\x1B[2J\x1B[1;1H";

pub trait DisplayDriver {
    fn render(&mut self, screen: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]);
}

#[derive(Default)]
pub struct TerminalDisplay {
    prev_frame: String,
}

impl DisplayDriver for TerminalDisplay {
    fn render(&mut self, screen: &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        let frame = screen
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "█" } else { " " })
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
