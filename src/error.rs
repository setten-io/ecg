use thiserror::Error;

pub(crate) type LcdResult<T> = std::result::Result<T, LcdError>;
pub(crate) type ConfigResult<T> = std::result::Result<T, ConfigError>;

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
pub(crate) enum LcdError {
    #[error("couldn't parse lcd response: {0}")]
    InvalidResponse(#[from] std::io::Error),
    #[error("couldn't fetch from lcd: {0}")]
    Lcd(#[from] ureq::Error),
}

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("couldn't read config: {0}")]
    Read(#[from] std::io::Error),
    #[error("couldn't parse config: {0}")]
    Parse(#[from] serde_yaml::Error),
}
