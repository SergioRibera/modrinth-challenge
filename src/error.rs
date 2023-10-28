use thiserror::Error;

pub type MResult<T> = Result<T, Modrinth>;

#[derive(Error, Debug)]
pub enum Modrinth {
    #[error("Request Fail: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Request Header is Invalid: {0}")]
    RequestInvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("IO: {0}")]
    RequestIO(#[from] std::io::Error),
    #[error("Failed to parse from String: {0}")]
    RequestUtf8(#[from] std::string::FromUtf8Error),
    #[error("Failed to parse '{0}'")]
    RequestInvalidParse(String),
}
