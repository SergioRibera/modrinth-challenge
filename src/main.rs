use std::env;

use error::Modrinth as MError;
use reqwest::Client;
use services::create_client;
use term::{accept, ask, note, show_project, welcome, MyBool};

mod error;
mod models;
mod services;
mod term;

pub static MODRINTH_API_KEY: &str = env!("MODRINTH_API_KEY");

#[tokio::main]
async fn main() -> Result<(), MError> {
    let client = create_client()?;

    welcome();

    if let Some(slug) = env::args().nth(1) {
        note("Note: we detect the parameter you shared as argument");
        process(&client, slug).await?;
    } else {
        loop {
            let slug = ask("Type the project slug", |_s| true)?;
            process(&client, slug).await?;
            let ok = ask::<MyBool>("Want to try another project? [Yes|No]", accept)?;

            if !ok.0 {
                break;
            }
        }
    }

    Ok(())
}

async fn process(client: &Client, slug: String) -> Result<(), MError> {
    let project = services::projects::get(client, slug).await?;
    show_project(&project);

    Ok(())
}
