use thiserror::Error;

pub(crate) type LcdResult<T> = std::result::Result<T, LcdError>;

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
pub(crate) enum LcdError {
    #[error("couldn't parse lcd response: {0}")]
    InvalidResponse(#[from] std::io::Error),
    #[error("couldn't fetch from lcd: {0}")]
    Lcd(#[from] ureq::Error),
}
