use anyhow::Error;
use chrono::Utc;
use reqwest;
use roxmltree;
use sqlx::{Pool, Postgres};

use crate::page::Page;

const MAX_TITLE_LENGTH: usize = 190;
const ELLIPSES_LENGTH: usize = 3;

// fetch_sitemap is a helper function to fetch sitemap from a given url
pub async fn fetch_sitemap(sitemap_url: &str) -> Result<Vec<String>, Error> {
    let mut urls: Vec<String> = Vec::new();

    let body = get_url_body(sitemap_url).await?;

    let xml = roxmltree::Document::parse(&body).unwrap();

    for url in xml.descendants().filter(|n| n.has_tag_name("loc")) {
        urls.push(url.text().unwrap().to_string());
    }

    Ok(urls)
}

pub async fn save_location(
    site: &str,
    loc: &String,
    pool: &Pool<Postgres>,
) -> Result<(), Error> {
    // sea_orm

    // sqlx
    let page = sqlx::query_as!(
        Page,
        r#"
        SELECT *
        FROM pages
        WHERE location = $1 AND site = $2
        "#,
        loc,
        site
    )
    .fetch_optional(pool)
    .await?;

    // if it's already in the database, then we skip
    if page.is_some() {
        return Ok(());
    }

    println!("Inserting page: {:?}", loc);
    let current_time = Utc::now();

    let _ = sqlx::query!(
        r#"
        INSERT INTO pages (
            site, is_approved, is_ignored,
            location, page_type, created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        site,
        false,
        false,
        loc,
        "General",
        current_time,
        current_time
    )
    .execute(pool)
    .await?;

    println!("Page inserted: {:?}", loc);
    Ok(())
}

pub async fn get_url_body(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

pub fn normalize_title(title: String) -> String {
    let title = title.trim().to_string();
    if title.len() <= MAX_TITLE_LENGTH {
        return title;
    }

    let mut new_title = String::new();
    let words: Vec<&str> = title.split(" ").collect();

    for word in words {
        if new_title.len() + word.len() > (MAX_TITLE_LENGTH - ELLIPSES_LENGTH) {
            break;
        }

        new_title.push_str(word);
        new_title.push_str(" ");
    }

    // remove last space
    new_title.pop();
    new_title.push_str("...");

    new_title
}
