use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, EcgError>;

#[derive(Error, Debug)]
pub(crate) enum EcgError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("couldn't query LCD: {0}")]
    Lcd(String),
}
