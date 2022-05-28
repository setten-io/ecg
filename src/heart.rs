use std::{thread::sleep, time::Duration};

pub struct Heart {
    interval: Duration,
}

impl Heart {
    pub fn new(interval: u64) -> Self {
        Self {
            interval: Duration::from_secs(interval),
        }
    }

    pub fn beat(self) {
        loop {
            println!("beat");
            sleep(self.interval);
        }
    }
}
