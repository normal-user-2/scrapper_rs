mod ext_web;
mod scrapper;

use ext_web::ext_web::ExtWeb;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

const DB_NAME: &str = "learning_rust";
const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db().await;

    let ews = ExtWeb::new(db);

    ews.sync().await;

    Ok(())
}

async fn get_db() -> DatabaseConnection {
    let db = Database::connect(&format!("{}/{}", DATABASE_URL, DB_NAME))
        .await
        .expect("failed to connect to db");

    Migrator::up(&db, None).await.expect("failed to migrate");

    db
}
