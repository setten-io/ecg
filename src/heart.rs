use std::{thread::sleep, time::Duration};

use crate::checkable::Checkable;

pub struct Heart {
    agent: ureq::Agent,
    interval: Duration,
    checkables: Vec<Box<dyn Checkable>>,
}

impl Heart {
    pub fn new(agent: ureq::Agent, interval: u64, checkables: Vec<Box<dyn Checkable>>) -> Self {
        Self {
            agent,
            interval: Duration::from_secs(interval),
            checkables,
        }
    }

    pub fn start(&mut self) {
        loop {
            let mut res = true;
            for checkable in &mut self.checkables {
                match checkable.check(&self.agent) {
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
            println!("beat {}", res);
            sleep(self.interval);
        }
    }
}
