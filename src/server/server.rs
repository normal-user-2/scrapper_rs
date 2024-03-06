use sqlx::{Pool, Postgres};

use axum::Router;

use crate::ext_web::ext_web::ExtWeb;

use super::routes;

pub struct Server {
    pool: Pool<Postgres>,
    ews: ExtWeb,
    router: Router,
}

impl Server {
    pub fn new(pool: Pool<Postgres>, ews: ExtWeb) -> Server {
        Server {
            pool,
            ews,
            router: routes::set_routes(),
        }
    }

    pub async fn start(self, port: &str) -> Result<(), anyhow::Error> {
        println!("starting the server on port: {}", port);
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
