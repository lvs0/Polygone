use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolygoneError {
    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

impl From<serde_json::Error> for PolygoneError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

impl From<bincode::Error> for PolygoneError {
    fn from(e: bincode::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}
