use std::time::{Instant, Duration};
use std::ops::Drop;

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn stop(self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("{}", self.start.elapsed().as_millis());
    }
}

