mod args;
mod constants;
mod core;
mod drivers;
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

    let mut display = TerminalDisplay::new(terminal, args.fps);
    let mut input = TerminalKeyboardInput::default();
    let mut audio = TerminalAudio::default();

    let mut chip8 = Chip8::new();
    chip8.load(rom.as_slice());
    chip8.run(args.clk_freq, &mut display, &mut input, &mut audio);

    restore_terminal();
}
