use std::convert::identity;

use self::response::SigningInfosResponse;

use crate::checkable::Checkable;
use crate::error::{EcgError, Result};

static PATH: &str = "/cosmos/slashing/v1beta1/signing_infos/";

#[derive(Debug, Default, Clone)]
pub(crate) struct SigningInfos {
    missed_blocks_counter: Option<u64>,
}

impl SigningInfos {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn is_tombstoned(&self, signing_infos: &SigningInfosResponse) -> bool {
        if !signing_infos.val_signing_info.tombstoned {
            log::debug!("tombstoned ok");
            return true;
        }
        log::warn!("validator is tombstoned");
        false
    }

    fn missed_blocks_counter_increased(&mut self, signing_infos: &SigningInfosResponse) -> bool {
        let missed_blocks_counter = signing_infos.val_signing_info.missed_blocks_counter;
        match self.missed_blocks_counter {
            Some(last_missed_blocks_counter) => {
                if missed_blocks_counter <= last_missed_blocks_counter {
                    log::debug!("misses ok");
                    return true;
                }
                log::warn!("missed blocks increased");
                self.missed_blocks_counter = Some(missed_blocks_counter);
                false
            }
            None => {
                log::debug!("misses warming up");
                self.missed_blocks_counter = Some(missed_blocks_counter);
                false
            }
        }
    }
}

impl Checkable for SigningInfos {
    fn check(&mut self, http: &ureq::Agent, url: &str, valcons_addr: &str) -> Result<bool> {
        let mut results: Vec<bool> = Vec::new();
        match http.get(&format!("{}{}{}", url, PATH, valcons_addr)).call() {
            Ok(res) => match res.into_json::<SigningInfosResponse>() {
                Ok(signing_infos) => [
                    self.is_tombstoned(&signing_infos),
                    self.missed_blocks_counter_increased(&signing_infos),
                ]
                .into_iter()
                .for_each(|r| results.push(r)),
                Err(e) => return Err(EcgError::Lcd(e.to_string())),
            },
            Err(e) => return Err(EcgError::Lcd(e.to_string())),
        }
        Ok(results.into_iter().all(identity))
    }
}

mod response {
    use serde::{Deserialize, Serialize};
    use serde_aux::field_attributes::deserialize_number_from_string;

    #[derive(Serialize, Deserialize)]
    pub(crate) struct SigningInfosResponse {
        pub(crate) val_signing_info: SigningInfos,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct SigningInfos {
        pub(crate) jailed_until: String,
        pub(crate) tombstoned: bool,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub(crate) missed_blocks_counter: u64,
    }
}
