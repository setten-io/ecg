use std::convert::identity;

use futures::{future, stream, StreamExt};
use tokio::time::{sleep, Duration};

use crate::client::{Client, ClientState};
use crate::electrode::Electrode;

pub(crate) struct Heart {
    clients: Vec<Box<dyn Client>>,
    http: reqwest::Client,
    heartbeat_url: String,
    electrodes: Vec<Box<dyn Electrode>>,
    interval: Duration,
}

impl Heart {
    pub(crate) fn new(
        clients: Vec<Box<dyn Client>>,
        http: reqwest::Client,
        heartbeat_url: String,
        electrodes: Vec<Box<dyn Electrode>>,
        interval: u64,
    ) -> Self {
        Self {
            clients,
            http,
            interval: Duration::from_secs(interval),
            heartbeat_url,
            electrodes,
        }
    }

    pub(crate) async fn start(&mut self) {
        log::info!("warming up");
        self.warm_up().await;
        loop {
            log::debug!("sleeping {:?}", self.interval);
            sleep(self.interval).await;
            let result = self.check().await;
            if result {
                log::info!("beating");
                self.beat().await;
                continue;
            }
            log::warn!("not beating");
        }
    }

    async fn warm_up(&mut self) {
        let state = match self.fresh_state().await {
            Some(state) => state,
            None => {
                log::error!("no state to warm up on");
                std::process::exit(1)
            }
        };

        stream::iter(&mut self.electrodes)
            .for_each(|e| async { e.warm_up(&state) })
            .await;
    }

    async fn check(&mut self) -> bool {
        let state = match self.fresh_state().await {
            Some(state) => state,
            None => {
                log::error!("no state found");
                return false;
            }
        };

        log::debug!("running all checks");
        stream::iter(&mut self.electrodes)
            .map(|e| e.measure(&state))
            .collect::<Vec<bool>>()
            .await
            .into_iter()
            .all(identity)
    }

    /// Get state from each client and return the freshest one (highest block height)
    async fn fresh_state(&self) -> Option<ClientState> {
        let states_futures: Vec<_> = self.clients.iter().map(|c| c.fetch()).collect();
        let states: Vec<ClientState> = match future::try_join_all(states_futures).await {
            Ok(states) => states,
            Err(e) => {
                log::error!("{}", e);
                std::process::exit(1);
            }
        };

        states.into_iter().min_by_key(|s| s.height)
    }

    async fn beat(&self) {
        if let Err(e) = self.http.get(&self.heartbeat_url).send().await {
            log::error!("couldn't beat: {}", e);
        }
    }
}
