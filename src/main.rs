mod ext_web;
mod page;
mod page_type;
mod scrapper;
mod site;

use anyhow::{anyhow, Error, Result};
use ext_web::ext_web::ExtWeb;
use sqlx::postgres::PgPoolOptions;
use std::{env, ops::Not};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // if environment is not set or it's set but it's not 'prod' or 'staging'
    // then we load envs from dev.env file
    if env::var("ENVIRONMENT")
        .map(|v| v != "prod" && v != "staging")
        .unwrap_or(true)
    {
        println!("loading .env");
        dotenvy::from_filename(".env").expect("failed to load .env");
    }

    if let Some(missing) = ensure_envs(&["ENVIRONMENT", "DATABASE_URL"]) {
        return Err(anyhow!("Missing env vars: {:?}", missing));
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let ews = ExtWeb::new(pool);

    ews.sync().await?;

    Ok(())
}

fn ensure_envs<'a>(envs: &'a [&'a str]) -> Option<Vec<&'a str>> {
    let out = envs
        .iter()
        .copied()
        .filter(|env| env::var(env).is_err())
        .collect::<Vec<_>>();
    out.is_empty().not().then_some(out)
}
