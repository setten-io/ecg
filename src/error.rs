use thiserror::Error;

#[derive(Error, Debug)]
pub enum EcgError {
    #[error("http error: {0}")]
    Http(#[from] ureq::Error),
}
