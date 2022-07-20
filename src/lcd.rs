use ureq::Agent;

use crate::error::LcdResult;

pub(crate) mod response;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) block: response::Block,
    pub(crate) signing_infos: response::SigningInfos,
}

#[derive(Debug)]
pub(crate) struct Client {
    http: ureq::Agent,
    url: String,
    valcons_addr: String,
}

impl Client {
    pub(crate) fn new(http: Agent, url: String, valcons_addr: String) -> Self {
        Self {
            http,
            url,
            valcons_addr,
        }
    }

    pub(crate) fn fetch(&self) -> LcdResult<State> {
        Ok(State {
            block: self.fetch_block()?,
            signing_infos: self.fetch_signing_infos()?,
        })
    }

    fn fetch_block(&self) -> LcdResult<response::Block> {
        let url = format!("{}/cosmos/base/tendermint/v1beta1/blocks/latest", self.url);
        let res = self.http.get(&url).call()?;
        Ok(res.into_json::<response::Block>()?)
    }

    fn fetch_signing_infos(&self) -> LcdResult<response::SigningInfos> {
        let url = format!(
            "{}/cosmos/slashing/v1beta1/signing_infos/{}",
            self.url, self.valcons_addr
        );
        let res = self.http.get(&url).call()?;
        Ok(res.into_json::<response::SigningInfos>()?)
    }
}
