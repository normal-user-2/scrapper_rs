use super::helper;
use crate::site::JFL;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use sqlx::{Pool, Postgres};

pub async fn sync_locations(pool: &Pool<Postgres>) {
    let mut urls = helper::fetch_sitemap("https://journeyforlight.wordpress.com/sitemap.xml").await;

    // skip url containing: journeyforlight.files.wordpress.com
    urls.retain(|url| !url.contains("journeyforlight.files.wordpress.com"));

    for url in &urls {
        helper::save_location(JFL, url, &pool).await;
    }
}

pub async fn extract_title(body: &String) -> String {
    let mut title = String::new();

    let document = Document::from(body.as_str());

    // .entry-header h1.entry-title
    for node in document.find(
        Attr("class", "entry-header")
            .descendant(Name("h1"))
            .descendant(Class("entry-title")),
    ) {
        title = node.text();
        break;
    }

    title
}
