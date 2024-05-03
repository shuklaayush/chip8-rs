use std::collections::VecDeque;

use crate::keypad::Key;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Press,
    Release,
}

#[derive(Clone, Copy)]
pub struct InputEvent {
    pub key: Key,
    pub kind: InputKind,
}

pub trait InputQueue {
    fn enqueue(&mut self, event: InputEvent, clk: u64);
    fn dequeue(&mut self, current_clk: u64) -> Option<InputEvent>;
}

impl InputQueue for VecDeque<(InputEvent, u64)> {
    fn enqueue(&mut self, event: InputEvent, clk: u64) {
        self.push_back((event, clk));
    }

    fn dequeue(&mut self, current_clk: u64) -> Option<InputEvent> {
        if let Some((_, clk)) = self.front() {
            if *clk <= current_clk {
                let (event, _) = self.pop_front().unwrap();
                Some(event)
            } else {
                None
            }
        } else {
            None
        }
    }
}
