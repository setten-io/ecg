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

    pub fn start(&mut self) -> ! {
        log::info!("warming up");
        self.checkables.iter_mut().for_each(|c| {
            log::debug!("warming");
            c.check(&self.agent, &self.lcd_url).unwrap();
        });
        sleep(self.interval);
        loop {
            let mut res = true;
            for checkable in &mut self.checkables {
                match checkable.check(&self.agent, &self.lcd_url) {
                    Err(e) => {
                        log::error!("{}", e);
                        res = false;
                    }
                    Ok(check_res) => {
                        if !check_res {
                            res = false
                        }
                    }
                }
            }
            if res {
                match self.agent.get(&self.heartbeat_url).call() {
                    Ok(_) => log::info!("beat"),
                    Err(e) => log::error!("couldn't beat, {}", e),
                }
            }
            sleep(self.interval);
        }
    }
}
