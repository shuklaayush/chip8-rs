mod constants;
mod core;
mod drivers;

use constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use core::Chip8;
use crossterm::terminal;
use std::{env, fs};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }

    // Check terminal size
    let (width, height) = terminal::size().expect("Failed to get terminal size");
    if width < 2 * DISPLAY_WIDTH as u16 {
        println!(
            "Error: Terminal width {width} less than minimum width {}",
            2 * DISPLAY_WIDTH
        );
        return;
    } else if height < DISPLAY_HEIGHT as u16 {
        println!("Error: Terminal height {height} less than minimum height {DISPLAY_HEIGHT}");
        return;
    }

    let path = args[1].as_str();
    let rom = fs::read(path).expect("Unable to read {path}");

    let mut display = TerminalDisplay::new(60);
    let mut input = TerminalKeyboardInput::new();
    let mut audio = TerminalAudio::default();

    let mut chip8 = Chip8::new();
    chip8.load(rom.as_slice());
    chip8.run(&mut display, &mut input, &mut audio);
}
