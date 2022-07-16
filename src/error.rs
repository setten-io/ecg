use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, EcgError>;

#[derive(Error, Debug)]
pub(crate) enum EcgError {
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
