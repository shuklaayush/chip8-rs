pub const NUM_REGISTERS: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const STACK_DEPTH: usize = 16;
pub const OPCODE_SIZE: u16 = 2;

pub const FLAG_REGISTER: usize = 0xF;

pub const FONTSET_START_ADDRESS: u16 = 0x0;
pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub const NUM_KEYS: usize = 16;
pub const KEYMAP_HEX: [usize; NUM_KEYS] = [
    0x1, 0x2, 0x3, 0xC, // 1 2 3 C
    0x4, 0x5, 0x6, 0xD, // 4 5 6 D
    0x7, 0x8, 0x9, 0xE, // 7 8 9 E
    0xA, 0x0, 0xB, 0xF, // A 0 B F
];

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const FONT_SIZE: usize = 5;
const NUM_FONTS: usize = 16;
#[rustfmt::skip]
pub const FONTSET: [u8; NUM_FONTS * FONT_SIZE] = [
    // 0
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,
    // 1
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
    // 2
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,
    // 3
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 4
    0b10010000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,
    // 5
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 6
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 7
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,
    // 8
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 9
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // A
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,
    // B
    0b11100000,
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,
    // C
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,
    // D
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,
    // E
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,
    // F
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000,
];

pub const TIMER_FREQ: u64 = 60; // 60Hz
