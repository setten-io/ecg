use self::response::BlockResponse;

use crate::checkable::Checkable;
use crate::error::Result;

static PATH: &str = "/cosmos/base/tendermint/v1beta1/blocks/latest";

#[derive(Debug, Clone)]
pub struct Block {
    last_height: Option<u64>,
}

impl Default for Block {
    fn default() -> Self {
        Self { last_height: None }
    }
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    fn height_increased(&mut self, block: BlockResponse) -> bool {
        let height = block.block.header.height;
        match self.last_height {
            Some(last_height) => {
                if height > last_height {
                    self.last_height = Some(height);
                    return true;
                }
                false
            }
            None => {
                self.last_height = Some(height);
                false
            }
        }
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
