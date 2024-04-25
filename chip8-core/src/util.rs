use std::{
    sync::{Arc, RwLock},
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::error::Chip8Error;

#[inline]
pub(crate) fn run_loop(
    status: Arc<RwLock<Result<(), Chip8Error>>>,
    frequency: u64,
    mut fn_tick: impl FnMut(Duration) -> Result<(), Chip8Error>,
) -> Result<(), Chip8Error> {
    let interval = if frequency > 0 {
        Duration::from_secs_f64(1.0 / frequency as f64)
    } else {
        Duration::ZERO
    };

    let mut prev_time = SystemTime::now();
    while status.read().unwrap().is_ok() {
        let curr_time = SystemTime::now();
        let elapsed = curr_time.duration_since(prev_time).unwrap_or_default();

        if elapsed >= interval {
            fn_tick(elapsed)?;
            prev_time = curr_time;
        } else {
            sleep(
                interval
                    .checked_sub(elapsed)
                    .unwrap_or_default()
                    .mul_f64(0.8),
            );
        }
    }

    Ok(())
}
