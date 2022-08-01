use std::time::Duration;

use async_trait::async_trait;
use reqwest::header;

use super::{Client, ClientState};
use crate::error::ClientResult;

pub(crate) struct Lcd {
    http: reqwest::Client,
    url: String,
    valcons_addr: String,
}

impl Lcd {
    pub(crate) fn new(http: reqwest::Client, url: String, valcons_addr: String) -> Self {
        Self {
            http,
            url,
            valcons_addr,
        }
    }

    pub(crate) fn new_setten(
        project_id: String,
        key: String,
        network: String,
        blockchain: String,
        valcons_addr: String,
    ) -> Self {
        let url = format!(
            "https://lcd.{}.{}.setten.io/{}",
            network, blockchain, project_id
        );

        let mut headers = header::HeaderMap::new();
        let key_header_value = match format!("Bearer {}", key).parse() {
            Ok(header_value) => header_value,
            Err(e) => {
                log::error!("incorrect http header value: {}", e);
                std::process::exit(1)
            }
        };
        headers.insert(header::AUTHORIZATION, key_header_value);

        let http = match reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .default_headers(headers)
            .build()
        {
            Ok(http) => http,
            Err(e) => {
                log::error!("couldn't build http client: {}", e);
                std::process::exit(1)
            }
        };

        Self {
            http,
            url,
            valcons_addr,
        }
    }

    async fn fetch_block(&self) -> ClientResult<response::Block> {
        let url = format!("{}/cosmos/base/tendermint/v1beta1/blocks/latest", self.url);
        let res = self.http.get(&url).send().await?;
        Ok(res.json::<response::Block>().await?)
    }

    async fn fetch_signing_infos(&self) -> ClientResult<response::SigningInfos> {
        let url = format!(
            "{}/cosmos/slashing/v1beta1/signing_infos/{}",
            self.url, self.valcons_addr
        );
        let res = self.http.get(&url).send().await?;
        Ok(res.json::<response::SigningInfos>().await?)
    }
}

#[async_trait]
impl Client for Lcd {
    async fn fetch(&self) -> ClientResult<ClientState> {
        let block = self.fetch_block().await?;
        let signing_infos = self.fetch_signing_infos().await?;
        log::debug!("fetched state for {} at {}", self.valcons_addr, self.url);
        Ok(ClientState {
            height: block.block.header.height,
            _jailed: signing_infos.val_signing_info.jailed_until,
            tombstoned: signing_infos.val_signing_info.tombstoned,
            missed_blocks: signing_infos.val_signing_info.missed_blocks_counter,
        })
    }
}

mod response {
    use serde::{Deserialize, Serialize};
    use serde_aux::field_attributes::deserialize_number_from_string;

    #[derive(Serialize, Deserialize, Clone)]
    pub(crate) struct Block {
        pub(crate) block: Block_,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub(crate) struct Block_ {
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
}
