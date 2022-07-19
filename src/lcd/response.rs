use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Block {
    pub(crate) block: BlockData,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct BlockData {
    pub(crate) header: Header,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Header {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub(crate) height: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SigningInfos {
    pub(crate) val_signing_info: ValSigningInfos,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ValSigningInfos {
    pub(crate) jailed_until: String,
    pub(crate) tombstoned: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub(crate) missed_blocks_counter: u64,
}
