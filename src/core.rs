use super::constants::*;
use super::utils::*;

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

    fn fetch(&mut self) -> u16 {
        let pc = self.program_counter as usize;
        let op = u16::from_le_bytes([self.memory[pc], self.memory[pc + 1]]);
        self.program_counter += 2;

        op
    }

    fn execute(&mut self, op: u16) {
        let nibbles = (
            (op & 0xF000) >> 12 as u8,
            (op & 0x0F00) >> 8 as u8,
            (op & 0x00F0) >> 4 as u8,
            (op & 0x000F) as u8,
        );

        match nibbles {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => self.clear_screen(),
            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                // Pop from stack
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.stack_pointer -= 1;
            }
            // JMP NNN
            (0x1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.program_counter = nnn;
            }
            // CALL NNN
            (0x2, _, _, _) => {
                let nnn = op & 0xFFF;

                // Push to stack
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;

                self.program_counter = nnn;
            }
            // SKIP VX == NN
            (0x3, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.registers[x as usize] == nn {
                    self.program_counter += 2;
                }
            }
            // SKIP VX != NN
            (0x4, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.registers[x as usize] != nn {
                    self.program_counter += 2;
                }
            }
            // SKIP VX == VY
            (0x5, x, y, _) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += 2;
                }
            }
            // VX = NN
            (0x6, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.registers[x as usize] = nn;
            }
            // VX += NN
            (0x6, x, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.registers[x as usize] += nn;
            }
            // VX = VY
            (0x7, x, y, 0x0) => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            // VX |= VY
            (0x8, x, y, 0x1) => {
                self.registers[x as usize] |= self.registers[y as usize];
            }
            // VX &= VY
            (0x8, x, y, 0x2) => {
                self.registers[x as usize] &= self.registers[y as usize];
            }
            // VX ^= VY
            (0x8, x, y, 0x3) => {
                self.registers[x as usize] ^= self.registers[y as usize];
            }
            // VX += VY
            (0x8, x, y, 0x4) => {
                let (sum, carry) =
                    self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = sum;
                self.registers[FLAG_REGISTER] = carry as u8;
            }
            // VX -= VY
            (0x8, x, y, 0x5) => {
                let (diff, borrow) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = diff;
                self.registers[FLAG_REGISTER] = borrow as u8;
            }
            // VX >>= 1
            (0x8, x, _, 0x6) => {
                self.registers[x as usize] -= self.registers[x as usize] >> 1;
                self.registers[FLAG_REGISTER] = self.registers[x as usize] & 1;
            }
            // VX = VY - VX
            (0x8, x, y, 0x7) => {
                let (diff, borrow) =
                    self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                self.registers[x as usize] = diff;
                self.registers[FLAG_REGISTER] = borrow as u8;
            }
            // VX <<= 1
            (0x8, x, _, 0xE) => {
                self.registers[x as usize] -= self.registers[x as usize] << 1;
                self.registers[FLAG_REGISTER] = (self.registers[x as usize] >> 7) & 1;
            }
            // SKIP VX != VY
            (0x9, x, y, 0x0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += 2;
                }
            }
            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.index_register = nnn;
            }
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.program_counter = (self.registers[0] as u16) + nnn;
            }
            // VX = rand() & NN
            (0xC, x, _, _) => {
                let nn = op & 0xFF;
                self.registers[x as usize] = self.rng() & nn;
            }
            // DRAW
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
                        flipped |= pixel & self.display[y][x];
                        self.display[y][x] ^= pixel;
                    }
                }
                self.registers[FLAG_REGISTER] = flipped as u8;
            }
            // SKIP KEY PRESS
            (0xE, x, 0x9, 0xE) => {
                let vx = self.registers[x as usize];
                if self.keypad[vx as usize] {
                    self.program_counter += 2;
                }
            }
            // SKIP KEY RELEASE
            (0xE, x, 0x9, 0xE) => {
                let vx = self.registers[x as usize];
                if !self.keypad[vx as usize] {
                    self.program_counter += 2;
                }
            }
            // VX = DT
            (0xF, x, 0x0, 0x7) => {
                self.registers[x as usize] = self.delay_timer;
            }
            // WAIT KEY
            (0xF, x, 0x0, 0xA) => {
                let pressed = false;
                for (i, key) in self.keypad.into_iter().enumerate() {
                    if key {
                        self.registers[x as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.program_counter -= 2;
                }
            }
            // DT = VX
            (0xF, x, 0x1, 0x5) => {
                self.delay_timer = self.registers[x as usize];
            }
            // ST = VX
            (0xF, x, 0x1, 0x8) => {
                self.sound_timer = self.registers[x as usize];
            }
            // I += VX
            (0xF, x, 0x1, 0xE) => {
                let vx = self.registers[x as usize];
                self.index_register = self.index_register.wrapping_add(vx as u16);
            }
            // I = FONT
            (0xF, x, 0x2, 0x9) => {
                let vx = self.registers[x as usize];
                self.index_register = FONTSET_START_ADDRESS + (FONT_SIZE as u16) * (vx as u16);
            }
            // BCD
            (0xF, x, 0x3, 0x3) => {
                let vx = self.registers[x as usize];
                self.memory[self.index_register as usize] = (vx / 100) % 10;
                self.memory[self.index_register as usize + 1] = (vx / 10) % 10;
                self.memory[self.index_register as usize + 2] = vx % 10;
            }
            // STORE V0 - VX
            (0xF, x, 0x5, 0x5) => {
                for i in 0..=x as usize {
                    self.memory[self.index_register as usize + i] = self.registers[i];
                }
            }
            // LOAD V0 - VX
            (0xF, x, 0x6, 0x5) => {
                for i in 0..=x as usize {
                    self.registers[i] = self.memory[self.index_register as usize + i];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {op}"),
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // BEEP
            }
            self.sound_timer -= 1;
        }
    }

    fn clear_screen(&mut self) {
        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
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
