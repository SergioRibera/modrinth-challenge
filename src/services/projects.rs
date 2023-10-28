use reqwest::Client;

use crate::error::MResult;
use crate::models::project::Project;
use crate::services::API_BASE_URL;

const BASE_PATH: &str = "/project/";

fn build_url(extra: String) -> String {
    format!("{API_BASE_URL}{BASE_PATH}{extra}")
}

/// The main idea with separating this by modules is that in each module
/// of this folder would go the service corresponding to the consumption of each api,
/// the ideal would be to implement a structure with a trait,
/// but for the size of this project and the requirement to be a single request,
/// it is not worth implementing the trait.
pub async fn get(client: &Client, slug: String) -> MResult<Project> {
    let url = build_url(slug);
    let res = client.get(url).send().await?.json::<Project>().await?;
    Ok(res)
}
