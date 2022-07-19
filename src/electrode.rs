use crate::lcd;

pub(crate) trait Electrode {
    fn measure(&mut self, state: lcd::State) -> bool;
}

#[derive(Default, Debug)]
pub(crate) struct BlockHeight {
    last_height: Option<u64>,
}

impl Electrode for BlockHeight {
    fn measure(&mut self, state: lcd::State) -> bool {
        let height = state.block.block.header.height;
        let last_height = match self.last_height {
            Some(last_height) => last_height,
            None => {
                log::debug!("warmed up block height ({})", height);
                self.last_height = Some(height);
                return false;
            }
        };
        if height > last_height {
            log::debug!("block height ok ({} +{})", height, height - last_height);
            self.last_height = Some(height);
            return true;
        }
        log::warn!("block height not ok ({})", last_height);
        false
    }
}

#[derive(Default, Debug)]
pub(crate) struct Tombstoned {}

impl Electrode for Tombstoned {
    fn measure(&mut self, state: lcd::State) -> bool {
        let res = !state.signing_infos.val_signing_info.tombstoned;
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
    fn measure(&mut self, state: lcd::State) -> bool {
        let missed_blocks = state.signing_infos.val_signing_info.missed_blocks_counter;
        let last_missed_blocks = match self.last_missed_blocks {
            Some(last_missed_blocks) => last_missed_blocks,
            None => {
                log::debug!("warmed up missed blocks ({})", missed_blocks);
                self.last_missed_blocks = Some(missed_blocks);
                return false;
            }
        };
        if missed_blocks <= last_missed_blocks {
            log::debug!("missed blocks ok ({})", missed_blocks);
            return true;
        }
        log::warn!(
            "missed blocks not ok ({} +{})",
            missed_blocks,
            missed_blocks - last_missed_blocks
        );
        self.last_missed_blocks = Some(missed_blocks);
        false
    }
}
