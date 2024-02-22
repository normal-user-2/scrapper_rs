use super::{helper, jfl};
use crate::site::JFL;
use anyhow::{anyhow, Error, Ok};
use chrono::Utc;
use futures::TryStreamExt;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

pub struct ExtWeb {
    pool: Pool<Postgres>,
    sites: Vec<String>,
}

impl ExtWeb {
    pub fn new(pool: Pool<Postgres>) -> ExtWeb {
        ExtWeb {
            pool,
            sites: vec![JFL.to_string()],
        }
    }

    pub async fn sync(&self) -> Result<(), Error> {
        for site in &self.sites {
            // location
            self.sync_locations(site).await?;

            // source
            self.sync_sources(site).await?;
        }

        Ok(())
    }

    async fn sync_locations(&self, site: &str) -> Result<(), Error> {
        println!("Syncing locations for site: {:?}", site);

        match site {
            JFL => jfl::sync_locations(&self.pool).await?,
            _ => {
                println!("Site not found: {:?}", site);
            }
        };

        Ok(())
    }

    async fn sync_sources(&self, site: &str) -> Result<(), Error> {
        println!("{:?} syncing sources", site);

        let mut rows = sqlx::query(
            r#"
            SELECT *
            FROM pages
            WHERE site = $1
            ORDER by id ASC
        "#,
        )
        .bind(site)
        .fetch(&self.pool);

        let mut n = 1;

        while let Some(row) = rows.try_next().await? {
            // map the row into a user-defined domain type
            self.sync_source(row, site).await?;

            if n % 50 == 0 {
                println!("Synced {:?} pages", n);
            }

            n += 1;
        }

        Ok(())
    }

    async fn sync_source(&self, row: PgRow, site: &str) -> Result<(), Error> {
        let loc: &str = row.try_get("location")?;
        let synced_source: Option<String> = row.try_get("source")?;
        let synced_title: Option<String> = row.try_get("title")?;
        let id: i64 = row.try_get("id")?;

        let source: String = helper::get_url_body(loc).await?;

        if source.is_empty() {
            println!("{:?} failed to fetch source for page: {:?}", site, loc);
            return Err(anyhow!("Failed to fetch source for page: {:?}", loc));
        }

        if synced_source.is_some() && synced_source.unwrap().eq(&source) {
            if synced_title.is_none() {
                let title = self.get_title(&source, site)?;

                println!("{}, updating title: {}, loc: {}", site, title, loc);

                let _ = sqlx::query!(
                    r#"
                    UPDATE pages
                    SET title = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    title,
                    Utc::now(),
                    id,
                ).execute(&self.pool).await?;
            }

            return Ok(());
        }

        let title = self.get_title(&source, site)?;

        println!("{}, new source, title: {}, loc: {}", site, title, loc);

        // update source and page
        let _ = sqlx::query!(
            r#"
            UPDATE pages
            SET source = $1, title = $2, updated_at = $3
            WHERE id = $4
            "#,
            source,
            title,
            Utc::now(),
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    fn get_title(&self, source: &String, site: &str) -> Result<String, Error> {
        let mut title: String;
        match site {
            JFL => {
                title = jfl::extract_title(source);
            }
            _ => {
                return Err(anyhow!("Site not found: {:?}", site));
            }
        }
        title = helper::normalize_title(title);

        Ok(title)
    }
}
