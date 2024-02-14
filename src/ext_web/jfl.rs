use sea_orm::DatabaseConnection;

use super::helper;

pub async fn sync_locations(db: &DatabaseConnection) -> Vec<String> {
    let mut urls = helper::fetch_sitemap("https://journeyforlight.wordpress.com/sitemap.xml").await;

    // skip url containing: journeyforlight.files.wordpress.com
    urls.retain(|url| !url.contains("journeyforlight.files.wordpress.com"));

    for url in &urls {
        helper::save_location(entity::enums::Site::JFL, url, db).await;
    }

    urls
}
