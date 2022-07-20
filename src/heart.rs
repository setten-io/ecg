use std::convert::identity;
use std::thread::sleep;
use std::time::Duration;

use crate::electrode::Electrode;
use crate::lcd;

pub(crate) struct Heart {
    lcd: lcd::Client,
    http: ureq::Agent,
    heartbeat_url: String,
    electrodes: Vec<Box<dyn Electrode>>,
    interval: Duration,
}

impl Heart {
    pub(crate) fn new(
        lcd: lcd::Client,
        http: ureq::Agent,
        heartbeat_url: String,
        electrodes: Vec<Box<dyn Electrode>>,
        interval: u64,
    ) -> Self {
        Self {
            lcd,
            http,
            interval: Duration::from_secs(interval),
            heartbeat_url,
            electrodes,
        }
    }

    pub(crate) fn start(&mut self) {
        log::info!("warming up");
        self.check();
        loop {
            log::debug!("sleeping {:?}", self.interval);
            sleep(self.interval);
            let result = self.check();
            if result {
                log::info!("beating");
                self.beat();
                continue;
            }
            log::warn!("not beating");
        }
    }

    fn check(&mut self) -> bool {
        log::debug!("running all checks");
        let state = match self.lcd.fetch() {
            Ok(state) => state,
            Err(e) => {
                log::error!("{}", e);
                return false;
            }
        };
        self.electrodes
            .iter_mut()
            .map(|e| e.measure(state.clone()))
            // needed to ensure ALL "measures" are run
            // `all` stopes consuming at the first `false`
            .collect::<Vec<bool>>()
            .into_iter()
            .all(identity)
    }

    fn beat(&self) {
        if let Err(e) = self.http.get(&self.heartbeat_url).call() {
            log::error!("couldn't beat: {}", e);
        }
    }
}
