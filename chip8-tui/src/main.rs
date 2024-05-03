mod args;
mod drivers;
mod error;
mod terminal;

use args::CmdArgs;
use chip8_core::Chip8;
use clap::Parser;
use error::TuiError;
use rand::{random, rngs::StdRng, SeedableRng};
use std::fs;
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

async fn app() -> Result<(), TuiError> {
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

    let seeded_rng = StdRng::seed_from_u64(args.random_seed.unwrap_or(random()));
    let mut chip8 = Chip8::new(args.clk_freq, seeded_rng);
    let res = chip8
        .load_and_run(rom.as_slice(), input, display, audio)
        .await
        .map_err(TuiError::Chip8Error);

    restore_terminal(args.headless).map_err(|e| TuiError::TerminalRestoreError(e.to_string()))?;
    res
}

#[tokio::main]
async fn main() {
    if let Err(e) = app().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
