use rand::Rng;

use super::Cpu;
use crate::state::{SimpleState, Word};

pub struct SimpleCpu<R: Rng> {
    state: SimpleState,
    clk_freq: u64,
    rng: R,
}

impl<R: Rng> SimpleCpu<R> {
    pub fn new(clk_freq: u64, rng: R) -> Self {
        Self {
            state: SimpleState::default(),
            clk_freq,
            rng,
        }
    }
}

impl<R: Rng> Cpu for SimpleCpu<R> {
    type State = SimpleState;

    fn state(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn random(&mut self) -> Word {
        self.rng.gen()
    }

    fn frequency(&self) -> u64 {
        self.clk_freq
    }
}
