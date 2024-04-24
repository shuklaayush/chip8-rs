use rand::random;

use super::constants::*;
use crate::drivers::{audio::AudioDriver, display::DisplayDriver, input::InputDriver};

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
    frame_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
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
            frame_buffer: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
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

    pub fn load(&mut self, bytes: &[u8]) {
        let start = PROGRAM_START_ADDRESS as usize;
        let end = PROGRAM_START_ADDRESS as usize + bytes.len();
        self.memory[start..end].copy_from_slice(bytes);
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.program_counter as usize;

        u16::from_be_bytes([self.memory[pc], self.memory[pc + 1]])
    }

    fn execute(&mut self, op: u16) {
        // println!("0x{:X}", op);

        let nibbles = (
            (op & 0xF000) >> 12_u8,
            (op & 0x0F00) >> 8_u8,
            (op & 0x00F0) >> 4_u8,
            (op & 0x000F) as u8,
        );

        match nibbles {
            // CLS
            // 0x00E0
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear screen
                self.frame_buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // RET
            // 0x00EE
            (0x0, 0x0, 0xE, 0xE) => {
                // Pop return address from stack
                self.stack_pointer -= 1;
                // Jump to top of stack
                self.program_counter = self.stack[self.stack_pointer as usize];
            }
            // JMP NNN
            // 0x1NNN
            (0x1, _, _, _) => {
                let nnn = op & 0xFFF;
                // Jump to 0xNNN
                self.program_counter = nnn;
            }
            // CALL NNN
            // 0x2NNN
            (0x2, _, _, _) => {
                let nnn = op & 0xFFF;
                // Push address of next instruction to stack
                self.stack[self.stack_pointer as usize] = self.program_counter + OPCODE_SIZE;
                self.stack_pointer += 1;
                // Jump to 0xNNN
                self.program_counter = nnn;
            }
            // SKIP VX == NN
            // 0x3XNN
            (0x3, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.registers[x as usize] == nn {
                    self.program_counter += OPCODE_SIZE;
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // SKIP VX != NN
            // 0x4XNN
            (0x4, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.registers[x as usize] != nn {
                    self.program_counter += OPCODE_SIZE;
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // SKIP VX == VY
            // 0x5XY0
            (0x5, x, y, 0) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += OPCODE_SIZE;
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX = NN
            // 0x6XNN
            (0x6, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.registers[x as usize] = nn;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX += NN
            // 0x7XNN
            (0x7, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn);
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX = VY
            // 0x8XY0
            (0x8, x, y, 0x0) => {
                self.registers[x as usize] = self.registers[y as usize];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX |= VY
            // 0x8XY1
            (0x8, x, y, 0x1) => {
                self.registers[x as usize] |= self.registers[y as usize];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX &= VY
            // 0x8XY2
            (0x8, x, y, 0x2) => {
                self.registers[x as usize] &= self.registers[y as usize];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX ^= VY
            // 0x8XY3
            (0x8, x, y, 0x3) => {
                self.registers[x as usize] ^= self.registers[y as usize];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX += VY
            // 0x8XY4
            (0x8, x, y, 0x4) => {
                let (sum, carry) =
                    self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = sum;
                self.registers[FLAG_REGISTER] = carry as u8;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX -= VY
            // 0x8XY5
            (0x8, x, y, 0x5) => {
                let (diff, borrow) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = diff;
                self.registers[FLAG_REGISTER] = !borrow as u8;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX >>= 1
            // 0x8XY6
            (0x8, x, _, 0x6) => {
                self.registers[x as usize] >>= 1;
                self.registers[FLAG_REGISTER] = self.registers[x as usize] & 1;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX = VY - VX
            // 0x8XY7
            (0x8, x, y, 0x7) => {
                let (diff, borrow) =
                    self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                self.registers[x as usize] = diff;
                self.registers[FLAG_REGISTER] = !borrow as u8;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX <<= 1
            // 0x8XYE
            (0x8, x, _, 0xE) => {
                self.registers[x as usize] <<= 1;
                self.registers[FLAG_REGISTER] = (self.registers[x as usize] >> 7) & 1;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // SKIP VX != VY
            // 0x9XY0
            (0x9, x, y, 0x0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += OPCODE_SIZE;
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // I = NNN
            // 0xANNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.index_register = nnn;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // JMP V0 + NNN
            // 0xBNNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.program_counter = (self.registers[0] as u16) + nnn;
            }
            // VX = rand() & NN
            // 0xCNNN
            (0xC, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                let r: u8 = random();
                self.registers[x as usize] = r & nn;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // DRAW
            // 0xDXYN
            (0xD, x, y, n) => {
                let x0 = self.registers[x as usize];
                let y0 = self.registers[y as usize];
                let mut flipped = false;
                for ys in 0..n {
                    let y = (y0 + ys) as usize % DISPLAY_HEIGHT;
                    let pixels = self.memory[self.index_register as usize + ys as usize];
                    for xs in 0..8 {
                        let x = (x0 + xs) as usize % DISPLAY_WIDTH;
                        let pixel = (pixels >> (7 - xs)) & 1 == 1;
                        flipped |= pixel & self.frame_buffer[y][x];
                        self.frame_buffer[y][x] ^= pixel;
                    }
                }
                self.registers[FLAG_REGISTER] = flipped as u8;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // SKIP KEY PRESS
            // 0xEX9E
            (0xE, x, 0x9, 0xE) => {
                let vx = self.registers[x as usize];
                if self.keypad[vx as usize] {
                    self.program_counter += OPCODE_SIZE;
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // SKIP KEY RELEASE
            // 0xEXA1
            (0xE, x, 0xA, 0x1) => {
                let vx = self.registers[x as usize];
                if !self.keypad[vx as usize] {
                    self.program_counter += OPCODE_SIZE;
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // VX = DT
            // 0xFX07
            (0xF, x, 0x0, 0x7) => {
                self.registers[x as usize] = self.delay_timer;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // WAIT KEY
            // 0xFX0A
            (0xF, x, 0x0, 0xA) => {
                let mut pressed = false;
                for (i, key) in self.keypad.into_iter().enumerate() {
                    if key {
                        self.registers[x as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if pressed {
                    // Increment PC
                    self.program_counter += OPCODE_SIZE;
                }
            }
            // DT = VX
            // 0xFX15
            (0xF, x, 0x1, 0x5) => {
                self.delay_timer = self.registers[x as usize];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // ST = VX
            // 0xFX18
            (0xF, x, 0x1, 0x8) => {
                self.sound_timer = self.registers[x as usize];
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // I += VX
            // 0xFX1E
            (0xF, x, 0x1, 0xE) => {
                let vx = self.registers[x as usize];
                self.index_register = self.index_register.wrapping_add(vx as u16);
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // I = FONT
            // 0xFX29
            (0xF, x, 0x2, 0x9) => {
                let vx = self.registers[x as usize];
                self.index_register = FONTSET_START_ADDRESS + (FONT_SIZE as u16) * (vx as u16);
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // BCD
            // 0xFX33
            (0xF, x, 0x3, 0x3) => {
                let vx = self.registers[x as usize];
                self.memory[self.index_register as usize] = (vx / 100) % 10;
                self.memory[self.index_register as usize + 1] = (vx / 10) % 10;
                self.memory[self.index_register as usize + 2] = vx % 10;
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // STORE V0 - VX
            // 0xFX55
            (0xF, x, 0x5, 0x5) => {
                for i in 0..=x as usize {
                    self.memory[self.index_register as usize + i] = self.registers[i];
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            // LOAD V0 - VX
            // 0xFX65
            (0xF, x, 0x6, 0x5) => {
                for i in 0..=x as usize {
                    self.registers[i] = self.memory[self.index_register as usize + i];
                }
                // Increment PC
                self.program_counter += OPCODE_SIZE;
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: 0x{op:X}"),
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self, audio: &mut impl AudioDriver) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            audio.beep();
            self.sound_timer -= 1;
        }
    }

    pub fn run(
        &mut self,
        display: &mut impl DisplayDriver,
        input: &mut impl InputDriver,
        audio: &mut impl AudioDriver,
    ) {
        'chip: loop {
            for _ in 0..TICKS_PER_FRAME {
                match input.poll() {
                    Ok(keys) => self.keypad = keys,
                    Err(_) => break 'chip,
                }
                self.tick();
            }
            self.tick_timers(audio);

            display.draw(&self.frame_buffer);
        }
    }
}
