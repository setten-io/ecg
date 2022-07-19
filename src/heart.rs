use std::convert::identity;
use std::thread::sleep;
use std::time::Duration;

use crate::checkable::Checkable;

pub(crate) struct Heart {
    agent: ureq::Agent,
    interval: Duration,
    lcd_url: String,
    heartbeat_url: String,
    valcons_addr: String,
    checkables: Vec<Box<dyn Checkable>>,
}

impl Heart {
    pub(crate) fn new(
        agent: ureq::Agent,
        interval: u64,
        lcd_url: String,
        heartbeat_url: String,
        valcons_addr: String,
        checkables: Vec<Box<dyn Checkable>>,
    ) -> Self {
        Self {
            agent,
            interval: Duration::from_secs(interval),
            lcd_url,
            heartbeat_url,
            valcons_addr,
            checkables,
        }
    }

    pub(crate) fn start(&mut self) -> ! {
        log::info!("warming up");
        let _ = self.check();
        loop {
            log::debug!("sleeping {:?}", self.interval);
            sleep(self.interval);
            let result = self.check();
            self.beat(result);
        }
    }

    fn check(&mut self) -> bool {
        log::debug!("running all checks");
        let mut results: Vec<bool> = Vec::new();
        self.checkables.iter_mut().for_each(|c| {
            match c.check(&self.agent, &self.lcd_url, &self.valcons_addr) {
                Ok(res) => results.push(res),
                Err(e) => {
                    log::warn!("{}", e);
                    results.push(false)
                }
            };
        });
        let result = results.into_iter().all(identity);
        log::debug!("checks passed: {}", result);
        result
    }

    fn beat(&self, check_result: bool) {
        if check_result {
            if let Err(e) = self.agent.get(&self.heartbeat_url).call() {
                log::error!("couldn't beat: {}", e);
            } else {
                log::info!("beat")
            }
        } else {
            log::warn!("not beating");
        }
    }
}
