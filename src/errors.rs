use std::io::{self, ErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataParseError {
    #[error("Unexpected binary EOF")]
    UnexpectedEOF,
    #[error("Invalid conversion: {e}")]
    InvalidConversion { e: String },
    #[error("{e}")]
    Custom { e: String },
    #[error("IO error: {e}")]
    IoError {
        #[from]
        e: io::Error,
    },
    #[cfg(feature = "crypto")]
    #[error("Crypto error: {e}")]
    CryptoError { e: String },
}

impl From<DataParseError> for io::Error {
    fn from(value: DataParseError) -> Self {
        Self::new(ErrorKind::Other, value)
    }
}
