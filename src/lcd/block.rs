use self::response::BlockResponse;

use crate::checkable::Checkable;
use crate::error::{EcgError, Result};

static PATH: &str = "/cosmos/base/tendermint/v1beta1/blocks/latest";

#[derive(Debug, Default, Clone)]
pub(crate) struct Block {
    last_height: Option<u64>,
}

impl Block {
    pub(crate) fn new() -> Self {
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
                log::warn!("height didn't increase");
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
    fn check(&mut self, http: &ureq::Agent, url: &str, _: &str) -> Result<bool> {
        match http.get(&format!("{}{}", url, PATH)).call() {
            Ok(res) => match res.into_json::<BlockResponse>() {
                Ok(block) => Ok(self.height_increased(block)),
                Err(e) => Err(EcgError::Lcd(e.to_string())),
            },
            Err(e) => Err(EcgError::Lcd(e.to_string())),
        }
    }
}

mod response {
    use serde::{Deserialize, Serialize};
    use serde_aux::field_attributes::deserialize_number_from_string;

    #[derive(Serialize, Deserialize)]
    pub(crate) struct BlockResponse {
        pub(crate) block: Block,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct Block {
        pub(crate) header: Header,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct Header {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub(crate) height: u64,
    }
}
