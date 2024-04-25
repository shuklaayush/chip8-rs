use crossterm::{
    cursor::{Hide, Show},
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use std::io::{stdout, Error, Stdout};
use chip8_core::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(
        stdout(),
        EnterAlternateScreen,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES),
        Hide,
    )?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;

    // Check terminal size
    let Rect { width, height, .. } = terminal.size()?;
    if width < 2 * DISPLAY_WIDTH as u16 {
        Error::other(format!(
            "Error: Terminal width {width} less than minimum width {}",
            2 * DISPLAY_WIDTH,
        ));
    } else if height < DISPLAY_HEIGHT as u16 {
        Error::other(format!(
            "Error: Terminal height {height} less than minimum height {DISPLAY_HEIGHT}"
        ));
    }

    Ok(terminal)
}

pub fn restore_terminal() {
    execute!(
        stdout(),
        Show,
        PopKeyboardEnhancementFlags,
        LeaveAlternateScreen
    )
    .expect("Failed to execute terminal commands");
    disable_raw_mode().expect("Failed to disable raw mode");
}
