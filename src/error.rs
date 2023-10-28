use thiserror::Error;

pub type MResult<T> = Result<T, Modrinth>;

#[derive(Error, Debug)]
pub enum Modrinth {
    #[error("Request Fail: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Request Fail: {0}")]
    RequestInvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
}
