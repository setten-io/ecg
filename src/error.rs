use thiserror::Error;

pub type EcgResult<T> = Result<T, EcgError>;

#[derive(Error, Debug)]
pub enum EcgError {
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
