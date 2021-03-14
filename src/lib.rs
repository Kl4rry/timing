use std::time::{Instant, Duration};

pub struct Timer<'a> {
    start: Instant,
    label: Option<&'a str>,
}

impl<'a> Timer<'a> {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
            label: None,
        }
    }

    pub fn with_label(label: &'a str) -> Self {
        Self {
            start: Instant::now(),
            label: Some(label),
        }
    }

    pub fn stop(self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn print_secs(&self) {
        if let Some(label) = self.label {
            println!("{}: {}s", label, self.start.elapsed().as_secs());
        } else {
            println!("{}s", self.start.elapsed().as_secs());
        }
    }

    pub fn print_ms(&self) {
        if let Some(label) = self.label {
            println!("{}: {}ms", label, self.start.elapsed().as_millis());
        } else {
            println!("{}ms", self.start.elapsed().as_millis());
        }
    }

    pub fn print_micros(&self) {
        if let Some(label) = self.label {
            println!("{}: {}μs", label, self.start.elapsed().as_micros());
        } else {
            println!("{}μs", self.start.elapsed().as_micros());
        }
    }

    pub fn print_nanos(&self) {
        if let Some(label) = self.label {
            println!("{}: {}ns", label, self.start.elapsed().as_nanos());
        } else {
            println!("{}ns", self.start.elapsed().as_nanos());
        }
    }
}

