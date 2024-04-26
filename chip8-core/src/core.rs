use rand::random;
use std::sync::{Arc, RwLock};

use crate::{
    constants::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, FONTSET, FONTSET_START_ADDRESS, FONT_SIZE,
        MEMORY_SIZE, NUM_KEYS, NUM_REGISTERS, OPCODE_SIZE, PROGRAM_START_ADDRESS, STACK_DEPTH,
        TIMER_FREQ,
    },
    drivers::{AudioDriver, DisplayDriver, InputDriver},
    error::Chip8Error,
    instructions::Instruction,
    rwlock::{CheckedRead, CheckedWrite},
    util::run_loop,
};

pub struct Chip8 {
    registers: [u8; NUM_REGISTERS],
    memory: [u8; MEMORY_SIZE],
    index_register: u16,
    program_counter: u16,
    stack: [u16; STACK_DEPTH],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: Arc<RwLock<u8>>,
    keypad: Arc<[RwLock<bool>; NUM_KEYS]>,
    frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
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
            sound_timer: Arc::new(RwLock::new(0)),
            keypad: Arc::new([false; NUM_KEYS].map(RwLock::new)),
            frame_buffer: Arc::new(RwLock::new([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT])),
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

    pub fn load(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        let start = PROGRAM_START_ADDRESS as usize;
        let end = PROGRAM_START_ADDRESS as usize + bytes.len();

        if end > MEMORY_SIZE {
            Err(Chip8Error::RomTooBig(bytes.len()))
        } else {
            self.memory[start..end].copy_from_slice(bytes);
            Ok(())
        }
    }

    fn fetch(&mut self) -> Result<u16, Chip8Error> {
        let pc = self.program_counter as usize;
        if pc + 1 > MEMORY_SIZE {
            Err(Chip8Error::ProgramCounterOverflow(self.program_counter))
        } else {
            Ok(u16::from_be_bytes([self.memory[pc], self.memory[pc + 1]]))
        }
    }

    pub fn decode(opcode: u16) -> Result<Instruction, Chip8Error> {
        let x = ((opcode >> 8) & 0x000F) as usize;
        let y = ((opcode >> 4) & 0x000F) as usize;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0xF0FF {
                // 0x00E0
                0x00E0 => Ok(Instruction::ClearDisplay),
                // 0x00EE
                0x00EE => Ok(Instruction::Return),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x1NNN
            0x1000 => Ok(Instruction::Jump(nnn)),
            // 0x2NNN
            0x2000 => Ok(Instruction::Call(nnn)),
            // 0x3XNN
            0x3000 => Ok(Instruction::SkipEqual(x, nn)),
            // 0x4XNN
            0x4000 => Ok(Instruction::SkipNotEqual(x, nn)),
            // 0x5XY0
            0x5000 => match opcode & 0xF00F {
                0x5000 => Ok(Instruction::SkipEqualXY(x, y)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x6XNN
            0x6000 => Ok(Instruction::Load(x, nn)),
            // 0x7XNN
            0x7000 => Ok(Instruction::Add(x, nn)),
            0x8000 => match opcode & 0xF00F {
                // 0x8XY0
                0x8000 => Ok(Instruction::Move(x, y)),
                // 0x8XY1
                0x8001 => Ok(Instruction::Or(x, y)),
                // 0x8XY2
                0x8002 => Ok(Instruction::And(x, y)),
                // 0x8XY3
                0x8003 => Ok(Instruction::Xor(x, y)),
                // 0x8XY4
                0x8004 => Ok(Instruction::AddXY(x, y)),
                // 0x8XY5
                0x8005 => Ok(Instruction::SubXY(x, y)),
                // TODO: Check
                // 0x8XY6
                0x8006 => Ok(Instruction::ShiftRight(x)),
                // 0x8XY7
                0x8007 => Ok(Instruction::SubYX(x, y)),
                // 0x8XYE
                0x800E => Ok(Instruction::ShiftLeft(x)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0x9000 => match opcode & 0xF00F {
                // 0x9XY0
                0x9000 => Ok(Instruction::SkipNotEqualXY(x, y)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0xANNN
            0xA000 => Ok(Instruction::LoadI(nnn)),
            // 0xBNNN
            0xB000 => Ok(Instruction::JumpV0(nnn)),
            // 0xCXNN
            0xC000 => Ok(Instruction::Random(x, nn)),
            // 0xDXYN
            0xD000 => Ok(Instruction::Draw(x, y, n)),
            0xE000 => match opcode & 0xF0FF {
                // 0xEX9E
                0xE09E => Ok(Instruction::SkipKeyPressed(x)),
                // 0xEXA1
                0xE0A1 => Ok(Instruction::SkipKeyNotPressed(x)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },

            0xF000 => match opcode & 0xF0FF {
                // 0xFX07
                0xF007 => Ok(Instruction::LoadDelay(x)),
                // 0xFX0A
                0xF00A => Ok(Instruction::WaitKeyPress(x)),
                // 0xFX15
                0xF015 => Ok(Instruction::SetDelay(x)),
                // 0xFX18
                0xF018 => Ok(Instruction::SetSound(x)),
                // 0xFX1E
                0xF01E => Ok(Instruction::AddI(x)),
                // 0xFX29
                0xF029 => Ok(Instruction::LoadFont(x)),
                // 0xFX33
                0xF033 => Ok(Instruction::StoreBCD(x)),
                // 0xFX55
                0xF055 => Ok(Instruction::StoreRegisters(x)),
                // 0xFX65
                0xF065 => Ok(Instruction::LoadMemory(x)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::ClearDisplay => {
                let mut frame_buffer = self.frame_buffer.checked_write()?;
                *frame_buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Return => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }
            Instruction::Jump(nnn) => {
                self.program_counter = nnn;
            }
            Instruction::Call(nnn) => {
                self.stack[self.stack_pointer as usize] = self.program_counter + OPCODE_SIZE;
                self.stack_pointer += 1;
                self.program_counter = nnn;
            }
            Instruction::SkipEqual(x, nn) => {
                if self.registers[x] == nn {
                    self.program_counter += OPCODE_SIZE;
                }
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SkipNotEqual(x, nn) => {
                if self.registers[x] != nn {
                    self.program_counter += OPCODE_SIZE;
                }
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SkipEqualXY(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.program_counter += OPCODE_SIZE;
                }
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Load(x, nn) => {
                self.registers[x] = nn;
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Add(x, nn) => {
                self.registers[x] = self.registers[x].wrapping_add(nn);
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Move(x, y) => {
                self.registers[x] = self.registers[y];
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Or(x, y) => {
                self.registers[x] |= self.registers[y];
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::And(x, y) => {
                self.registers[x] &= self.registers[y];
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Xor(x, y) => {
                self.registers[x] ^= self.registers[y];
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::AddXY(x, y) => {
                let (sum, carry) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = sum;
                self.registers[FLAG_REGISTER] = carry as u8;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SubXY(x, y) => {
                let (diff, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = diff;
                self.registers[FLAG_REGISTER] = !borrow as u8;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::ShiftRight(x) => {
                self.registers[x] >>= 1;
                self.registers[FLAG_REGISTER] = self.registers[x] & 1;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SubYX(x, y) => {
                let (diff, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = diff;
                self.registers[FLAG_REGISTER] = !borrow as u8;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::ShiftLeft(x) => {
                self.registers[x] <<= 1;
                self.registers[FLAG_REGISTER] = (self.registers[x] >> 7) & 1;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SkipNotEqualXY(x, y) => {
                if self.registers[x] != self.registers[y] {
                    self.program_counter += OPCODE_SIZE;
                }

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::LoadI(nnn) => {
                self.index_register = nnn;
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::JumpV0(nnn) => {
                self.program_counter = (self.registers[0] as u16) + nnn;
            }
            Instruction::Random(x, nn) => {
                // TODO: See if random is portable/wasm-friendly
                let r: u8 = random();
                self.registers[x] = r & nn;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::Draw(x, y, n) => {
                let x0 = self.registers[x] as usize % DISPLAY_WIDTH;
                let y0 = self.registers[y] as usize % DISPLAY_HEIGHT;
                let mut flipped = false;
                let mut frame_buffer = (*self.frame_buffer).checked_write()?;
                for ys in 0..n {
                    let y = (y0 + ys as usize) % DISPLAY_HEIGHT;
                    let pixels = self.memory[self.index_register as usize + ys as usize];
                    for xs in 0..8 {
                        let x = (x0 + xs) % DISPLAY_WIDTH;
                        let pixel = (pixels >> (7 - xs)) & 1 == 1;
                        flipped |= pixel & frame_buffer[y][x];
                        frame_buffer[y][x] ^= pixel;
                    }
                }
                self.registers[FLAG_REGISTER] = flipped as u8;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SkipKeyPressed(x) => {
                let vx = self.registers[x];
                if *self.keypad[vx as usize].checked_read()? {
                    self.program_counter += OPCODE_SIZE;
                }
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SkipKeyNotPressed(x) => {
                let vx = self.registers[x];
                if !*self.keypad[vx as usize].checked_read()? {
                    self.program_counter += OPCODE_SIZE;
                }
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::LoadDelay(x) => {
                self.registers[x] = self.delay_timer;
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::WaitKeyPress(x) => {
                // TODO: Is this right? Better to halt thread and wait for key press?
                let mut pressed = false;
                for (i, key) in self.keypad.iter().enumerate() {
                    if *key.checked_read()? {
                        self.registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if pressed {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SetDelay(x) => {
                self.delay_timer = self.registers[x];
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::SetSound(x) => {
                *self.sound_timer.checked_write()? = self.registers[x];
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::AddI(x) => {
                let vx = self.registers[x];
                self.index_register = self.index_register.wrapping_add(vx as u16);

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::LoadFont(x) => {
                let vx = self.registers[x];
                self.index_register = FONTSET_START_ADDRESS + (FONT_SIZE as u16) * (vx as u16);

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::StoreBCD(x) => {
                let vx = self.registers[x];
                self.memory[self.index_register as usize] = (vx / 100) % 10;
                self.memory[self.index_register as usize + 1] = (vx / 10) % 10;
                self.memory[self.index_register as usize + 2] = vx % 10;

                self.program_counter += OPCODE_SIZE;
            }
            Instruction::StoreRegisters(x) => {
                for i in 0..=x {
                    self.memory[self.index_register as usize + i] = self.registers[i];
                }
                self.program_counter += OPCODE_SIZE;
            }
            Instruction::LoadMemory(x) => {
                for i in 0..=x {
                    self.registers[i] = self.memory[self.index_register as usize + i];
                }
                self.program_counter += OPCODE_SIZE;
            }
        }

        Ok(())
    }

    // Fetch -> Decode -> Execute
    pub fn tick(&mut self) -> Result<(), Chip8Error> {
        let op = self.fetch()?;
        let instruction = Self::decode(op)?;
        self.execute(instruction)
    }

    pub fn tick_timers(&mut self) -> Result<(), Chip8Error> {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if *self.sound_timer.checked_read()? > 0 {
            *self.sound_timer.checked_write()? -= 1;
        }

        Ok(())
    }

    fn run_cpu(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        clk_freq: u64,
        freq: Arc<RwLock<Option<f64>>>,
    ) {
        let ticks_per_timer = clk_freq / TIMER_FREQ;

        let mut clk = 0;
        run_loop(status, clk_freq, move |elapsed| {
            self.tick()?;

            if ticks_per_timer == 0 || clk % ticks_per_timer == 0 {
                self.tick_timers()?;
            }
            *freq.checked_write()? = Some(1.0 / elapsed.as_secs_f64());
            clk += 1;

            Ok(())
        })
    }

    pub async fn run(
        &mut self,
        clk_freq: u64,
        mut input: impl InputDriver + 'static,
        display: Option<impl DisplayDriver + 'static>,
        audio: Option<impl AudioDriver + 'static>,
    ) -> Result<(), Chip8Error> {
        // Status flag to check if machine is still running
        let status = Arc::new(RwLock::new(Ok(())));
        let freq = Arc::new(RwLock::new(Some(clk_freq as f64)));

        // Input loop
        let input_handle = {
            let status = status.clone();
            let keypad = self.keypad.clone();

            tokio::spawn(async move { input.run(status, keypad) })
        };
        // Render loop
        let display_handle = {
            display.map(|mut display| {
                let status = status.clone();
                let frame_buffer = self.frame_buffer.clone();
                let freq = freq.clone();

                tokio::spawn(async move { display.run(status, frame_buffer, freq) })
            })
        };
        // Audio loop
        let audio_handle = {
            audio.map(|mut audio| {
                let status = status.clone();
                let sound_timer = self.sound_timer.clone();
                tokio::spawn(async move { audio.run(status, sound_timer) })
            })
        };
        // CPU loop
        self.run_cpu(status.clone(), clk_freq, freq);

        // Wait for all threads
        input_handle
            .await
            .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()))?;
        if let Some(display_handle) = display_handle {
            display_handle
                .await
                .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()))?;
        }
        if let Some(audio_handle) = audio_handle {
            audio_handle
                .await
                .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()))?;
        }

        let res = status.checked_read()?;
        res.clone()
    }

    pub async fn load_and_run(
        &mut self,
        bytes: &[u8],
        clk_freq: u64,
        input: impl InputDriver + 'static,
        display: Option<impl DisplayDriver + 'static>,
        audio: Option<impl AudioDriver + 'static>,
    ) -> Result<(), Chip8Error> {
        self.load(bytes)?;
        self.run(clk_freq, input, display, audio).await
    }
}
