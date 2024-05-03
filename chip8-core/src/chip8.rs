use rand::{Rng, SeedableRng};
use std::sync::{Arc, RwLock};

use crate::{
    constants::{FONTSET, FONTSET_START_ADDRESS, MEMORY_SIZE, PROGRAM_START_ADDRESS},
    cpu::Cpu,
    drivers::{AudioDriver, DisplayDriver, InputDriver},
    error::Chip8Error,
    rwlock::CheckedRead,
    state::Chip8State,
};

pub struct Chip8<R: Rng + SeedableRng> {
    state: Chip8State,
    cpu: Cpu<R>,
}

impl<R: Rng + SeedableRng> Chip8<R> {
    pub fn new(cpu_freq: u64, rng: R) -> Self {
        let mut state = Chip8State {
            program_counter: PROGRAM_START_ADDRESS,
            ..Default::default()
        };
        // Load fontset
        let start = FONTSET_START_ADDRESS as usize;
        let end = FONTSET_START_ADDRESS as usize + FONTSET.len();
        state.memory[start..end].copy_from_slice(FONTSET.as_slice());

        Self {
            state,
            cpu: Cpu::new(cpu_freq, rng),
        }
    }

    pub fn load(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        let start = PROGRAM_START_ADDRESS as usize;
        let end = PROGRAM_START_ADDRESS as usize + bytes.len();

        if end > MEMORY_SIZE {
            Err(Chip8Error::RomTooBig(bytes.len()))
        } else {
            self.state.memory[start..end].copy_from_slice(bytes);
            Ok(())
        }
    }

    pub async fn run(
        &mut self,
        mut input: impl InputDriver + 'static,
        display: Option<impl DisplayDriver + 'static>,
        audio: Option<impl AudioDriver + 'static>,
    ) -> Result<(), Chip8Error> {
        // Status flag to check if machine is still running
        let status = Arc::new(RwLock::new(Ok(())));

        // Input loop
        let input_handle = {
            let status = status.clone();
            let keypad = self.state.keypad.clone();

            tokio::spawn(async move { input.run(status, keypad) })
        };
        // Render loop
        let display_handle = {
            display.map(|mut display| {
                let status = status.clone();
                let frame_buffer = self.state.frame_buffer.clone();
                let clk = self.state.clk.clone();

                tokio::spawn(async move { display.run(status, frame_buffer, clk) })
            })
        };
        // Audio loop
        let audio_handle = {
            audio.map(|mut audio| {
                let status = status.clone();
                let sound_timer = self.state.sound_timer.clone();
                tokio::spawn(async move { audio.run(status, sound_timer) })
            })
        };
        // CPU loop
        self.cpu.run(status.clone(), &mut self.state);

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
        rom: &[u8],
        input: impl InputDriver + 'static,
        display: Option<impl DisplayDriver + 'static>,
        audio: Option<impl AudioDriver + 'static>,
    ) -> Result<(), Chip8Error> {
        self.load(rom)?;
        self.run(input, display, audio).await
    }
}
