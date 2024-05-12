use rand::Rng;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FONTSET_START_ADDRESS, FONT_SIZE, TICKS_PER_TIMER},
    error::Chip8Error,
    input::{InputEvent, InputQueue},
    instructions::Instruction,
    rwlock::{CheckedRead, CheckedWrite},
    state::State,
    util::run_loop,
};

pub struct Cpu<R: Rng> {
    clk_freq: u64,
    rng: R,
}

impl<R: Rng> Cpu<R> {
    pub fn new(clk_freq: u64, rng: R) -> Self {
        Self { clk_freq, rng }
    }

    pub fn frequency(&self) -> u64 {
        self.clk_freq
    }

    fn fetch<S: State>(&mut self, state: &mut S) -> Result<u16, Chip8Error> {
        let pc = state.program_counter();
        let hi = state.memory(pc)?;
        let lo = state.memory(pc + 1)?;

        state.increment_program_counter();
        Ok(u16::from_be_bytes([hi, lo]))
    }

    fn decode(opcode: u16) -> Result<Instruction, Chip8Error> {
        let x = ((opcode >> 8) & 0x000F) as u8;
        let y = ((opcode >> 4) & 0x000F) as u8;

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

    fn execute<S: State>(
        &mut self,
        instruction: Instruction,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        state: &mut S,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::ClearDisplay => {
                state.clear_framebuffer()?;
            }
            Instruction::Return => {
                state.pop_stack();
            }
            Instruction::Jump(nnn) => {
                state.set_program_counter(nnn);
            }
            Instruction::Call(nnn) => {
                state.push_stack(nnn);
            }
            Instruction::SkipEqual(x, nn) => {
                let vx = state.register(x);
                if vx == nn {
                    state.increment_program_counter();
                }
            }
            Instruction::SkipNotEqual(x, nn) => {
                let vx = state.register(x);
                if vx != nn {
                    state.increment_program_counter();
                }
            }
            Instruction::SkipEqualXY(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                if vx == vy {
                    state.increment_program_counter();
                }
            }
            Instruction::Load(x, nn) => {
                state.set_register(x, nn);
            }
            Instruction::Add(x, nn) => {
                let vx = state.register(x);
                let val = vx.wrapping_add(nn);
                state.set_register(x, val);
            }
            Instruction::Move(x, y) => {
                let vy = state.register(y);
                state.set_register(x, vy);
            }
            Instruction::Or(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let val = vx | vy;
                state.set_register(x, val);
            }
            Instruction::And(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let val = vx & vy;
                state.set_register(x, val);
            }
            Instruction::Xor(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let val = vx ^ vy;
                state.set_register(x, val);
            }
            Instruction::AddXY(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let (sum, carry) = vx.overflowing_add(vy);

                state.set_register(x, sum);
                state.set_flag_register(carry);
            }
            Instruction::SubXY(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let (diff, borrow) = vx.overflowing_sub(vy);

                state.set_register(x, diff);
                state.set_flag_register(!borrow);
            }
            Instruction::ShiftRight(x) => {
                let vx = state.register(x);
                let flag = (vx & 1) != 0;
                let val = vx >> 1;

                state.set_flag_register(flag);
                state.set_register(x, val);
            }
            Instruction::SubYX(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let (diff, borrow) = vy.overflowing_sub(vx);

                state.set_register(x, diff);
                state.set_flag_register(!borrow);
            }
            Instruction::ShiftLeft(x) => {
                let vx = state.register(x);
                let flag = ((vx >> 7) & 1) != 0;
                let val = vx << 1;

                state.set_register(x, val);
                state.set_flag_register(flag);
            }
            Instruction::SkipNotEqualXY(x, y) => {
                let vx = state.register(x);
                let vy = state.register(y);
                if vx != vy {
                    state.increment_program_counter();
                }
            }
            Instruction::LoadI(nnn) => {
                state.set_index_register(nnn);
            }
            Instruction::JumpV0(nnn) => {
                let v0 = state.register(0);
                let offset = (v0 as u16) + nnn;
                state.set_program_counter(offset);
            }
            Instruction::Random(x, nn) => {
                let r: u8 = self.rng.gen();
                let val = r & nn;
                state.set_register(x, val);
            }
            Instruction::Draw(x, y, n) => {
                let vx = state.register(x);
                let vy = state.register(y);
                let vi = state.index_register();

                let x0 = vx as usize % DISPLAY_WIDTH;
                let y0 = vy as usize % DISPLAY_HEIGHT;
                let mut flipped = false;
                for ys in 0..n {
                    let y = (y0 + ys as usize) % DISPLAY_HEIGHT;
                    let pixels = state.memory(vi + ys as u16)?;
                    for xs in 0..8 {
                        let x = (x0 + xs) % DISPLAY_WIDTH;
                        let pixel = (pixels >> (7 - xs)) & 1 == 1;
                        let fb = state.frame_buffer(y, x)?;
                        flipped |= pixel & fb;
                        if pixel {
                            state.set_frame_buffer(y, x, !fb)?;
                        }
                    }
                }
                state.set_flag_register(flipped);
            }
            Instruction::SkipKeyPressed(x) => {
                let vx = state.register(x);
                if state.key(vx) {
                    state.increment_program_counter();
                }
            }
            Instruction::SkipKeyNotPressed(x) => {
                let vx = state.register(x);
                if !state.key(vx) {
                    state.increment_program_counter();
                }
            }
            Instruction::LoadDelay(x) => {
                let val = state.delay_timer();
                state.set_register(x, val);
            }
            Instruction::WaitKeyPress(x) => {
                let clk = state.clk()?;
                while status.checked_read()?.is_ok() {
                    if let Some(event) = (*input_queue.checked_write()?).dequeue(clk) {
                        state.set_key(event.key, event.kind);
                        state.set_register(x, event.key as u8);
                        break;
                    }
                }
            }
            Instruction::SetDelay(x) => {
                let vx = state.register(x);
                state.set_delay_timer(vx);
            }
            Instruction::SetSound(x) => {
                let vx = state.register(x);
                state.set_sound_timer(vx)?;
            }
            Instruction::AddI(x) => {
                let vx = state.register(x);
                let vi = state.index_register();
                let addr = vi.wrapping_add(vx as u16);
                state.set_index_register(addr);
            }
            Instruction::LoadFont(x) => {
                let vx = state.register(x);
                let addr = FONTSET_START_ADDRESS + (FONT_SIZE as u16) * (vx as u16);
                state.set_index_register(addr);
            }
            Instruction::StoreBCD(x) => {
                let vx = state.register(x);
                let vi = state.index_register();

                state.set_memory(vi, (vx / 100) % 10)?;
                state.set_memory(vi + 1, (vx / 10) % 10)?;
                state.set_memory(vi + 2, vx % 10)?;
            }
            Instruction::StoreRegisters(x) => {
                let vi = state.index_register();
                for j in 0..=x {
                    let vj = state.register(j);
                    state.set_memory(vi + j as u16, vj)?;
                }
            }
            Instruction::LoadMemory(x) => {
                let vi = state.index_register();
                for j in 0..=x {
                    let val = state.memory(vi + j as u16)?;
                    state.set_register(j, val);
                }
            }
        }

        Ok(())
    }

    pub fn tick<S: State>(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        state: &mut S,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) -> Result<(), Chip8Error> {
        let op = self.fetch(state)?;
        let instruction = Self::decode(op)?;
        self.execute(instruction, status, state, input_queue)
    }

    pub fn tick_timers<S: State>(&mut self, state: &mut S) -> Result<(), Chip8Error> {
        if state.delay_timer() > 0 {
            state.decrement_delay_timer();
        }
        if state.sound_timer()? > 0 {
            state.decrement_sound_timer()?;
        }
        Ok(())
    }

    pub fn run<S: State>(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        state: &mut S,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            let clk = state.clk()?;

            while let Some(event) = (*input_queue.checked_write()?).dequeue(clk) {
                state.set_key(event.key, event.kind);
            }

            // TODO: How do I remove this clone?
            self.tick(status.clone(), state, input_queue.clone())?;
            if clk % TICKS_PER_TIMER == 0 {
                self.tick_timers(state)?;
            }

            state.increment_clk()?;
            Ok(())
        })
    }
}
