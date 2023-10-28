use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};

use crate::error::MResult;
use crate::MODRINTH_API_KEY;

pub mod projects;

pub static API_BASE_URL: &str = env!("API_BASE_URL");
const APP_USER_AGENT: &str = concat!(
    "SergioRibera",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

pub fn create_client() -> MResult<Client> {
    let mut headers = HeaderMap::new();
    headers
        .insert(
            "Authorization",
            HeaderValue::from_str(MODRINTH_API_KEY)?,
        );

    Ok(ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()?)
}
