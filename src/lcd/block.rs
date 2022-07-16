use self::response::BlockResponse;

use crate::checkable::Checkable;
use crate::error::Result;

static PATH: &str = "/cosmos/base/tendermint/v1beta1/blocks/latest";

#[derive(Debug, Clone)]
pub struct Block {
    last_height: u64,
}

impl Block {
    pub fn new() -> Self {
        Self { last_height: 0 }
    }

    fn height_increased(&mut self, block: BlockResponse) -> bool {
        let height = block.block.header.height;
        if height > self.last_height {
            self.last_height = height;
                    return true;
                }
                false
    }
}

impl Checkable for Block {
    fn check(&mut self, http: &ureq::Agent) -> Result<bool> {
        let block = http
            .get(&format!("https://phoenix-lcd.terra.dev{}", PATH))
            .call()?
            .into_json::<BlockResponse>()?;
        Ok(self.height_increased(block))
    }
}

mod response {
    use serde::{Deserialize, Serialize};
    use serde_aux::field_attributes::deserialize_number_from_string;

    #[derive(Serialize, Deserialize)]
    pub struct BlockResponse {
        pub block: Block,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Block {
        pub header: Header,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Header {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub height: u64,
    }
}
