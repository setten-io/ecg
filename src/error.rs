use thiserror::Error;

pub(crate) type ClientResult<T> = std::result::Result<T, ClientError>;
pub(crate) type ConfigResult<T> = std::result::Result<T, ConfigError>;

#[derive(Error, Debug)]
pub(crate) enum ClientError {
    #[error("couldn't parse client response: {0}")]
    InvalidResponse(#[from] std::io::Error),
    #[error("couldn't fetch with client: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("couldn't parse block timestamp: {0}")]
    TimestampError(#[from] chrono::ParseError),
}

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("couldn't read config: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("couldn't parse config: {0}")]
    ParseError(#[from] serde_yaml::Error),
}
