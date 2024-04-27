use std::sync::{Arc, RwLock};

use crate::constants::{
    DISPLAY_HEIGHT, DISPLAY_WIDTH, MEMORY_SIZE, NUM_KEYS, NUM_REGISTERS, OPCODE_SIZE, STACK_DEPTH,
};

pub type Address = u16;
pub type Word = u8;

pub struct Chip8State {
    pub registers: [Word; NUM_REGISTERS],
    pub memory: [Word; MEMORY_SIZE],
    pub index_register: Address,
    pub program_counter: Address,
    pub stack: [Address; STACK_DEPTH],
    pub stack_pointer: Word,
    pub delay_timer: Word,
    pub sound_timer: Arc<RwLock<Word>>,
    pub keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    pub frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
    /// Cycle counter to keep track of the number of CPU cycles executed.
    pub clk: Arc<RwLock<u64>>,
}

impl Default for Chip8State {
    fn default() -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
            memory: [0; MEMORY_SIZE],
            index_register: 0,
            program_counter: 0,
            stack: [0; STACK_DEPTH],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: Arc::new(RwLock::new(0)),
            keypad: Arc::new([false; NUM_KEYS].map(RwLock::new)),
            frame_buffer: Arc::new(RwLock::new([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT])),
            clk: Arc::new(RwLock::new(0)),
        }
    }
}

impl Chip8State {
    pub fn push_stack(&mut self, addr: Address) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = addr;
    }

    pub fn pop_stack(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += OPCODE_SIZE;
    }
}
