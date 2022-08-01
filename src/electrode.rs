use chrono::{DateTime, Utc};

use crate::client::ClientState;

pub(crate) trait Electrode {
    fn warm_up(&mut self, state: &ClientState);
    fn measure(&mut self, state: &ClientState) -> bool;
}

#[derive(Default, Debug)]
pub(crate) struct BlockHeight {
    last_height: Option<u64>,
}

impl Electrode for BlockHeight {
    fn warm_up(&mut self, state: &ClientState) {
        log::debug!("warmed up block height ({})", state.height);
        self.last_height = Some(state.height);
    }

    fn measure(&mut self, state: &ClientState) -> bool {
        let last_height = match self.last_height {
            Some(last_height) => last_height,
            None => {
                log::error!("block height was not initialized");
                return false;
            }
        };

        if state.height > last_height {
            log::debug!(
                "block height ok ({} +{})",
                state.height,
                state.height - last_height
            );
            self.last_height = Some(state.height);
            return true;
        }

        log::warn!("block height not ok ({})", last_height);
        false
    }
}

#[derive(Default, Debug)]
pub(crate) struct Tombstoned {}

impl Electrode for Tombstoned {
    fn warm_up(&mut self, _: &ClientState) {
        log::debug!("warmed up tombstoned (nothing to do)");
    }

    fn measure(&mut self, state: &ClientState) -> bool {
        let res = !state.tombstoned;
        match res {
            true => log::debug!("tombstoned ok (not tombstoned)"),
            false => log::warn!("tombstoned not ok (tombstoned)"),
        }
        res
    }
}

#[derive(Default, Debug)]
pub(crate) struct MissedBlocks {
    last_missed_blocks: Option<u64>,
}

impl Electrode for MissedBlocks {
    fn warm_up(&mut self, state: &ClientState) {
        log::debug!("warmed up missed blocks ({})", state.missed_blocks);
        self.last_missed_blocks = Some(state.missed_blocks);
    }

    fn measure(&mut self, state: &ClientState) -> bool {
        let last_missed_blocks = match self.last_missed_blocks {
            Some(last_missed_blocks) => last_missed_blocks,
            None => {
                log::error!("missed blocks was not initialized");
                return false;
            }
        };

        if state.missed_blocks <= last_missed_blocks {
            log::debug!("missed blocks ok ({})", state.missed_blocks);
            return true;
        }

        log::warn!(
            "missed blocks not ok ({} +{})",
            state.missed_blocks,
            state.missed_blocks - last_missed_blocks
        );
        self.last_missed_blocks = Some(state.missed_blocks);
        false
    }
}

#[derive(Default, Debug)]
pub(crate) struct Jailed {
    last_jailed_until: Option<DateTime<Utc>>,
}

impl Electrode for Jailed {
    fn warm_up(&mut self, state: &ClientState) {
        log::debug!("warmed up jailed ({})", state.jailed_until);
        self.last_jailed_until = Some(state.jailed_until);
    }

    fn measure(&mut self, state: &ClientState) -> bool {
        let last_jailed_until = match self.last_jailed_until {
            Some(last_jailed_until) => last_jailed_until,
            None => {
                log::error!("missed blocks was not initialized");
                return false;
            }
        };

        if state.jailed_until > last_jailed_until {
            self.last_jailed_until = Some(state.jailed_until);
        }

        let now = Utc::now();

        if state.jailed_until < now {
            log::debug!("jailed ok (not jailed since {})", state.jailed_until);
            return true;
        }

        log::warn!("jailed not ok (jailed until {})", state.jailed_until);
        false
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{self, DateTime, Utc};

    #[test]
    fn compare_block_time() {
        let time1 = "2022-08-01T16:18:53.169944174Z";
        let time2 = "2023-08-01T16:18:53.169944174Z";
        let parsed1: DateTime<Utc> = chrono::DateTime::from_str(time1).unwrap();
        let parsed2: DateTime<Utc> = chrono::DateTime::from_str(time2).unwrap();
        assert!(parsed1 < parsed2);
        assert_ne!(parsed1, parsed2);
    }
}
