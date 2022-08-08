use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::error::ClientResult;

pub(crate) mod lcd;

#[derive(Clone)]
pub(crate) struct ClientState {
    pub(crate) height: u64,
    pub(crate) jailed_until: DateTime<Utc>,
    pub(crate) tombstoned: bool,
    pub(crate) missed_blocks: u64,
}

#[async_trait]
pub(crate) trait Client {
    async fn fetch(&self, name: &str) -> ClientResult<ClientState>;
}
