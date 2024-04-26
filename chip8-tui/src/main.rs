mod args;
mod drivers;
mod error;
mod terminal;

use args::CmdArgs;
use chip8_core::core::Chip8;
use clap::Parser;
use error::TuiError;
use std::fs;
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

#[tokio::main]
async fn main() -> Result<(), TuiError> {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom).map_err(|e| TuiError::RomReadError(e.to_string()))?;
    let terminal =
        setup_terminal(args.headless).map_err(|e| TuiError::TerminalSetupError(e.to_string()))?;

    let input = TerminalKeyboardInput::default();
    let display = {
        if !args.headless {
            Some(TerminalDisplay::new(
                terminal,
                args.refresh_rate,
                args.bg_color,
                args.fg_color,
                args.border_color,
            ))
        } else {
            None
        }
    };
    let audio = {
        if !args.headless {
            Some(TerminalAudio::default())
        } else {
            None
        }
    };

    let mut chip8 = Chip8::new();
    let res = chip8
        .load_and_run(rom.as_slice(), args.clk_freq, input, display, audio)
        .await
        .map_err(TuiError::Chip8Error);

    restore_terminal(args.headless).map_err(|e| TuiError::TerminalRestoreError(e.to_string()))?;
    res
}
