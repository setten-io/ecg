use self::response::SigningInfosResponse;

use crate::checkable::Checkable;
use crate::error::{EcgError, Result};

static PATH: &str = "/cosmos/slashing/v1beta1/signing_infos/";

#[derive(Debug, Default, Clone)]
pub(crate) struct SigningInfos {}

impl SigningInfos {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn is_tombstoned(&self, signing_infos: SigningInfosResponse) -> bool {
        if !signing_infos.val_signing_info.tombstoned {
            return true;
        }
        log::warn!("validator is tombstoned");
        false
    }
}

impl Checkable for SigningInfos {
    fn check(&mut self, http: &ureq::Agent, url: &str, valcons_addr: &str) -> Result<bool> {
        match http.get(&format!("{}{}{}", url, PATH, valcons_addr)).call() {
            Ok(res) => match res.into_json::<SigningInfosResponse>() {
                Ok(signing_infos) => Ok(self.is_tombstoned(signing_infos)),
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
