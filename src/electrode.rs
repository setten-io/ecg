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
    fn warm_up(&mut self, state: &ClientState) {
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
