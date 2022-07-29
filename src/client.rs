use async_trait::async_trait;

use crate::error::LcdResult;

pub(crate) mod lcd;

#[derive(Clone)]
pub(crate) struct ClientState {
    pub(crate) height: u64,
    pub(crate) jailed: String,
    pub(crate) tombstoned: bool,
    pub(crate) missed_blocks: u64,
}

#[async_trait]
pub(crate) trait Client {
    async fn fetch(&self) -> LcdResult<ClientState>;
}
