mod constants;
mod utils;

use constants::*;
use utils::*;

pub struct Chip8 {
    registers: [u8; NUM_REGISTERS],
    memory: [u8; MEMORY_SIZE],
    index_register: u16,
    program_counter: u16,
    stack: [u16; STACK_DEPTH],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; NUM_KEYS],
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
            memory: [0; MEMORY_SIZE],
            index_register: 0,
            program_counter: 0,
            stack: [0; STACK_DEPTH],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; NUM_KEYS],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Self {
            program_counter: PROGRAM_START_ADDRESS,
            ..Default::default()
        };
        // Load fontset
        let start = FONTSET_START_ADDRESS as usize;
        let end = FONTSET_START_ADDRESS as usize + FONTSET.len();
        chip.memory[start..end].copy_from_slice(FONTSET.as_slice());

        chip
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn load(&mut self, bytes: &[u8]) {
        let start = PROGRAM_START_ADDRESS as usize;
        let end = PROGRAM_START_ADDRESS as usize + bytes.len();
        self.memory[start..end].copy_from_slice(bytes);
    }

    pub fn render(&self) {
        clear_screen();

        for row in self.display {
            for pixel in row {
                if pixel {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() {}
