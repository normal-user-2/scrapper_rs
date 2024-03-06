mod ext_web;
mod page;
mod page_type;
mod scrapper;
mod server;
mod site;

use anyhow::{anyhow, Result};
use ext_web::ext_web::ExtWeb;
use server::server::Server;
use sqlx::postgres::PgPoolOptions;
use std::{env, ops::Not, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // if environment is not set or it's set but it's not 'prod' or 'staging'
    // then we load envs from dev.env file
    if env::var("ENVIRONMENT")
        .map(|v| v != "prod" && v != "staging")
        .unwrap_or(true)
    {
        println!("loading .env");
        dotenvy::from_filename(".env").expect("failed to load .env");
    }

    if let Some(missing) = ensure_envs(&["ENVIRONMENT", "DATABASE_URL", "PORT"]) {
        return Err(anyhow!("Missing env vars: {:?}", missing));
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let server = Server::new(pool.clone(), ExtWeb::new(pool.clone()));

    server.start(&env::var("PORT")?).await?;

    // let str = Arc::new(String::from("asdf"));
    // let str: Arc<String> = Arc::new("asdf".to_string());
    // let str: Arc<String> = Arc::new("asdf".into());

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
