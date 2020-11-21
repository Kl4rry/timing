use std::time::{Instant, Duration};
use std::ops::Drop;
use std::time;
use std::thread;

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

#[test]
fn it_works() {
    let timer = Timer::start();
    thread::sleep(time::Duration::from_millis(10));
    assert_eq!(timer.stop().as_millis(), 10);
}

