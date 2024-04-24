mod audio;
mod constants;
mod core;
mod display;
mod input;

use core::Chip8;
use std::{env, fs};

use audio::TerminalAudio;
use display::TerminalDisplay;
use input::KeyboardInput;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
    }

    let path = args[1].as_str();
    let rom = fs::read(path).expect("Unable to read {path}");

    let mut display = TerminalDisplay::default();
    let mut input = KeyboardInput::default();
    let mut audio = TerminalAudio::default();

    let mut chip8 = Chip8::new();
    chip8.load(rom.as_slice());
    chip8.run(&mut display, &mut input, &mut audio);
}
