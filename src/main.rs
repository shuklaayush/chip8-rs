mod constants;
mod core;
mod utils;

use core::Chip8;
use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
    }

    let path = args[1].as_str();
    let rom = fs::read(path).expect("Unable to read {path}");

    let mut chip8 = Chip8::new();
    chip8.load(rom.as_slice());
    chip8.run();
}
