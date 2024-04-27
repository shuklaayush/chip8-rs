use std::sync::{Arc, RwLock};

use crate::constants::{
    DISPLAY_HEIGHT, DISPLAY_WIDTH, MEMORY_SIZE, NUM_KEYS, NUM_REGISTERS, STACK_DEPTH,
};

pub struct Chip8State {
    pub registers: [u8; NUM_REGISTERS],
    pub memory: [u8; MEMORY_SIZE],
    pub index_register: u16,
    pub program_counter: u16,
    pub stack: [u16; STACK_DEPTH],
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: Arc<RwLock<u8>>,
    pub keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    pub frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
    /// Cycle counter to keep track of the number of CPU cycles executed.
    pub clk: u64,
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
            clk: 0,
        }
    }
}
