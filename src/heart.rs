use std::{thread::sleep, time::Duration};

use crate::checkable::Checkable;

pub struct Heart {
    agent: ureq::Agent,
    interval: Duration,
    lcd_url: String,
    heartbeat_url: String,
    checkables: Vec<Box<dyn Checkable>>,
}

impl Heart {
    pub fn new(
        agent: ureq::Agent,
        interval: u64,
        lcd_url: String,
        heartbeat_url: String,
        checkables: Vec<Box<dyn Checkable>>,
    ) -> Self {
        Self {
            agent,
            interval: Duration::from_secs(interval),
            lcd_url,
            heartbeat_url,
            checkables,
        }
    }

    pub fn start(&mut self) {
        loop {
            let mut res = true;
            for checkable in &mut self.checkables {
                match checkable.check(&self.agent, &self.lcd_url) {
                    Err(e) => {
                        println!("error: {}", e);
                        res = false;
                    }
                    Ok(check_res) => {
                        if !check_res {
                            res = false
                        }
                    }
                }
            }
            self.agent.get(&self.heartbeat_url);
            log::info!("beat {}", res);
            sleep(self.interval);
        }
    }
}
