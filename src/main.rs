mod constants;
mod core;
mod utils;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
    }
}
