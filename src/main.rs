mod args;
mod constants;
mod core;
mod drivers;
mod error;
mod terminal;

use args::CmdArgs;
use clap::Parser;
use core::Chip8;
use std::fs;
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

fn main() {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom).expect("Unable to read {path}");
    let terminal = setup_terminal().expect("Failed to setup terminal");

    let mut display = TerminalDisplay::new(terminal, args.refresh_rate);
    let mut input = TerminalKeyboardInput::default();
    let mut audio = TerminalAudio::default();

    let mut chip8 = Chip8::new();
    match chip8.load_and_run(
        rom.as_slice(),
        args.clk_freq,
        &mut display,
        &mut input,
        &mut audio,
    ) {
        Ok(_) => {
            restore_terminal();
        }
        Err(e) => {
            restore_terminal();
            print!("{e}")
        }
    };
}
