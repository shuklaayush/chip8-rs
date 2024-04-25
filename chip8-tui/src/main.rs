mod args;
mod drivers;
mod terminal;

use args::CmdArgs;
use chip8_core::core::Chip8;
use clap::Parser;
use std::fs;
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom).expect("Unable to read {path}");
    let terminal = setup_terminal().expect("Failed to setup terminal");

    let display = TerminalDisplay::new(terminal, args.refresh_rate);
    let input = TerminalKeyboardInput::default();
    let audio = TerminalAudio::default();

    let mut chip8 = Chip8::new();
    match chip8
        .load_and_run(rom.as_slice(), args.clk_freq, display, input, audio)
        .await
    {
        Ok(_) => {
            restore_terminal();
        }
        Err(e) => {
            restore_terminal();
            print!("{e}")
        }
    };
}
