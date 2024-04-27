use rand::random;
use std::sync::{Arc, RwLock};

use crate::{
    constants::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, FONTSET_START_ADDRESS, FONT_SIZE,
        OPCODE_SIZE, TIMER_FREQ,
    },
    error::Chip8Error,
    instructions::Instruction,
    rwlock::{CheckedRead, CheckedWrite},
    state::Chip8State,
    util::run_loop,
};

pub struct Cpu {
    clk_freq: u64,
}

impl Cpu {
    pub fn new(clk_freq: u64) -> Self {
        Self { clk_freq }
    }

    pub fn frequency(&self) -> u64 {
        self.clk_freq
    }

    fn fetch(&mut self, state: &mut Chip8State) -> Result<u16, Chip8Error> {
        let pc = state.program_counter as usize;
        let hi = *state
            .memory
            .get(pc)
            .ok_or(Chip8Error::MemoryOutOfBounds(state.program_counter))?;
        let lo = *state
            .memory
            .get(pc + 1)
            .ok_or(Chip8Error::MemoryOutOfBounds(state.program_counter + 1))?;

        state.increment_pc();
        Ok(u16::from_be_bytes([hi, lo]))
    }

    fn decode(opcode: u16) -> Result<Instruction, Chip8Error> {
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

    fn execute(
        &mut self,
        state: &mut Chip8State,
        instruction: Instruction,
    ) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::ClearDisplay => {
                let mut frame_buffer = state.frame_buffer.checked_write()?;
                *frame_buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
            }
            Instruction::Return => {
                state.pop_stack();
            }
            Instruction::Jump(nnn) => {
                state.program_counter = nnn;
            }
            Instruction::Call(nnn) => {
                state.push_stack(nnn);
            }
            Instruction::SkipEqual(x, nn) => {
                if state.registers[x] == nn {
                    state.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SkipNotEqual(x, nn) => {
                if state.registers[x] != nn {
                    state.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SkipEqualXY(x, y) => {
                if state.registers[x] == state.registers[y] {
                    state.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::Load(x, nn) => {
                state.registers[x] = nn;
            }
            Instruction::Add(x, nn) => {
                state.registers[x] = state.registers[x].wrapping_add(nn);
            }
            Instruction::Move(x, y) => {
                state.registers[x] = state.registers[y];
            }
            Instruction::Or(x, y) => {
                state.registers[x] |= state.registers[y];
            }
            Instruction::And(x, y) => {
                state.registers[x] &= state.registers[y];
            }
            Instruction::Xor(x, y) => {
                state.registers[x] ^= state.registers[y];
            }
            Instruction::AddXY(x, y) => {
                let (sum, carry) = state.registers[x].overflowing_add(state.registers[y]);
                state.registers[x] = sum;
                state.registers[FLAG_REGISTER] = carry as u8;
            }
            Instruction::SubXY(x, y) => {
                let (diff, borrow) = state.registers[x].overflowing_sub(state.registers[y]);
                state.registers[x] = diff;
                state.registers[FLAG_REGISTER] = !borrow as u8;
            }
            Instruction::ShiftRight(x) => {
                state.registers[x] >>= 1;
                state.registers[FLAG_REGISTER] = state.registers[x] & 1;
            }
            Instruction::SubYX(x, y) => {
                let (diff, borrow) = state.registers[y].overflowing_sub(state.registers[x]);
                state.registers[x] = diff;
                state.registers[FLAG_REGISTER] = !borrow as u8;
            }
            Instruction::ShiftLeft(x) => {
                state.registers[x] <<= 1;
                state.registers[FLAG_REGISTER] = (state.registers[x] >> 7) & 1;
            }
            Instruction::SkipNotEqualXY(x, y) => {
                if state.registers[x] != state.registers[y] {
                    state.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::LoadI(nnn) => {
                state.index_register = nnn;
            }
            Instruction::JumpV0(nnn) => {
                state.program_counter = (state.registers[0] as u16) + nnn;
            }
            Instruction::Random(x, nn) => {
                // TODO: See if random is portable/wasm-friendly
                let r: u8 = random();
                state.registers[x] = r & nn;
            }
            Instruction::Draw(x, y, n) => {
                let x0 = state.registers[x] as usize % DISPLAY_WIDTH;
                let y0 = state.registers[y] as usize % DISPLAY_HEIGHT;
                let mut flipped = false;
                let mut frame_buffer = (*state.frame_buffer).checked_write()?;
                for ys in 0..n {
                    let y = (y0 + ys as usize) % DISPLAY_HEIGHT;
                    let pixels = state.memory[state.index_register as usize + ys as usize];
                    for xs in 0..8 {
                        let x = (x0 + xs) % DISPLAY_WIDTH;
                        let pixel = (pixels >> (7 - xs)) & 1 == 1;
                        flipped |= pixel & frame_buffer[y][x];
                        frame_buffer[y][x] ^= pixel;
                    }
                }
                state.registers[FLAG_REGISTER] = flipped as u8;
            }
            Instruction::SkipKeyPressed(x) => {
                let vx = state.registers[x];
                if *state.keypad[vx as usize].checked_read()? {
                    state.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SkipKeyNotPressed(x) => {
                let vx = state.registers[x];
                if !*state.keypad[vx as usize].checked_read()? {
                    state.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::LoadDelay(x) => {
                state.registers[x] = state.delay_timer;
            }
            Instruction::WaitKeyPress(x) => {
                // TODO: Is this right? Better to halt thread and wait for key press?
                let mut pressed = false;
                for (i, key) in state.keypad.iter().enumerate() {
                    if *key.checked_read()? {
                        state.registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    state.program_counter -= OPCODE_SIZE;
                }
            }
            Instruction::SetDelay(x) => {
                state.delay_timer = state.registers[x];
            }
            Instruction::SetSound(x) => {
                *state.sound_timer.checked_write()? = state.registers[x];
            }
            Instruction::AddI(x) => {
                let vx = state.registers[x];
                state.index_register = state.index_register.wrapping_add(vx as u16);
            }
            Instruction::LoadFont(x) => {
                let vx = state.registers[x];
                state.index_register = FONTSET_START_ADDRESS + (FONT_SIZE as u16) * (vx as u16);
            }
            Instruction::StoreBCD(x) => {
                let vx = state.registers[x];
                state.memory[state.index_register as usize] = (vx / 100) % 10;
                state.memory[state.index_register as usize + 1] = (vx / 10) % 10;
                state.memory[state.index_register as usize + 2] = vx % 10;
            }
            Instruction::StoreRegisters(x) => {
                for i in 0..=x {
                    state.memory[state.index_register as usize + i] = state.registers[i];
                }
            }
            Instruction::LoadMemory(x) => {
                for i in 0..=x {
                    state.registers[i] = state.memory[state.index_register as usize + i];
                }
            }
        }

        Ok(())
    }

    pub fn tick(&mut self, state: &mut Chip8State) -> Result<(), Chip8Error> {
        let op = self.fetch(state)?;
        let instruction = Self::decode(op)?;
        self.execute(state, instruction)
    }

    pub fn tick_timers(&mut self, state: &mut Chip8State) -> Result<(), Chip8Error> {
        if state.delay_timer > 0 {
            state.delay_timer -= 1;
        }
        if *state.sound_timer.checked_read()? > 0 {
            *state.sound_timer.checked_write()? -= 1;
        }
        Ok(())
    }

    pub fn run(
        &mut self,
        state: &mut Chip8State,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        freq: Arc<RwLock<Option<f64>>>,
    ) {
        let ticks_per_timer = self.frequency() / TIMER_FREQ;

        run_loop(status, self.frequency(), move |elapsed| {
            self.tick(state)?;

            if ticks_per_timer == 0 || state.clk % ticks_per_timer == 0 {
                self.tick_timers(state)?;
            }
            *freq.checked_write()? = Some(1.0 / elapsed.as_secs_f64());
            state.clk += 1;

            Ok(())
        })
    }
}
