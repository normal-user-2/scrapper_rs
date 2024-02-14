use entity::enums::Site;
use entity::page;
use entity::page::Entity as Page;
use reqwest;
use roxmltree;
use sea_orm::{prelude::TimeDate, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};
use chrono::{Utc, NaiveDateTime};


// fetch_sitemap is a helper function to fetch sitemap from a given url
pub async fn fetch_sitemap(sitemap_url: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();

    let resp = reqwest::get(sitemap_url).await;
    if resp.is_err() {
        return urls;
    }

    let body: String = resp.unwrap().text().await.unwrap();
    let xml = roxmltree::Document::parse(&body).unwrap();

    for url in xml.descendants().filter(|n| n.has_tag_name("loc")) {
        urls.push(url.text().unwrap().to_string());
    }

    urls
}

pub async fn save_location(site: Site, loc: &String, db: &sea_orm::DatabaseConnection) {
    let page = Page::find()
        .filter(
            Condition::all()
                .add(page::Column::Location.eq(loc.clone()))
                .add(page::Column::Site.eq(site.clone())),
        )
        .one(db)
        .await;


    match page {
        Ok(Some(page)) => {
            return;
        }
        Ok(None) => {
            println!("Inserting page: {:?}", loc);
            let current_time = Utc::now().naive_utc();

            let new_page = Page::insert(page::ActiveModel {
                site: Set(site.clone()),
                is_approved: Set(false),
                is_ignored: Set(false),
                location: Set(loc.clone()),
                page_type: Set(entity::enums::PageType::General),
                created_at: Set(current_time),
                updated_at: Set(current_time),
                ..Default::default()
            })
            .exec(db)
            .await;

            match new_page {
                Ok(_) => {
                    println!("Page inserted: {:?}", loc);
                }
                Err(e) => {
                    println!("Failed to insert page: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to find page: {:?}", e);
        }
    }
}
