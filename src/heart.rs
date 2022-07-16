use std::convert::identity;
use std::thread::sleep;
use std::time::Duration;

use crate::checkable::Checkable;

pub(crate) struct Heart {
    agent: ureq::Agent,
    interval: Duration,
    lcd_url: String,
    heartbeat_url: String,
    checkables: Vec<Box<dyn Checkable>>,
}

impl Heart {
    pub(crate) fn new(
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

    pub(crate) fn start(&mut self) -> ! {
        log::info!("warming up");
        self.check();
        loop {
            sleep(self.interval);
            let result = self.check();
            self.beat(result);
        }
    }

    fn check(&mut self) -> bool {
        let mut results =
            self.checkables
                .iter_mut()
                .map(|c| match c.check(&self.agent, &self.lcd_url) {
                    Ok(res) => res,
                    Err(e) => {
                        log::warn!("{}", e);
                        false
                    }
                });
        results.all(identity)
    }

    fn beat(&self, check_result: bool) {
        if check_result {
            match self.agent.get(&self.heartbeat_url).call() {
                Ok(_) => log::info!("beat"),
                Err(e) => log::error!("couldn't beat, {}", e),
            }
        } else {
            log::info!("not beating");
        }
    }
}
