mod args;
mod drivers;
mod terminal;

use args::CmdArgs;
use chip8_core::{core::Chip8, error::Chip8Error};
use clap::Parser;
use std::fs;
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

#[tokio::main]
async fn main() -> Result<(), Chip8Error> {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom).expect("Unable to read {path}");
    let terminal = setup_terminal().expect("Failed to setup terminal");

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
        .await;

    restore_terminal();
    res
}
