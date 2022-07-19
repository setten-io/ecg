use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("deserialization failed: {0}")]
    Deserialization(#[from] std::io::Error),
    #[error("http request failed: {0}")]
    Http(#[from] ureq::Error),
}
