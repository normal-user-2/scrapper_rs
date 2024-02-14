use chrono::Utc;
use entity::enums::Site;
use entity::page;
use entity::page::Entity as Page;
use reqwest;
use roxmltree;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

const MAX_TITLE_LENGTH: usize = 190;
const ELLIPSES_LENGTH: usize = 3;

// fetch_sitemap is a helper function to fetch sitemap from a given url
pub async fn fetch_sitemap(sitemap_url: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();

    let body = get_url_body(sitemap_url).await;

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
        Ok(Some(_page)) => {
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

pub async fn get_url_body(url: &str) -> String {
    let resp = reqwest::get(url).await;

    match resp {
        Ok(body) => {
            let body = body.text().await;
            match body {
                Ok(b) => {
                    return b;
                }
                Err(e) => {
                    println!("Failed to unwrap text: {:?}", e);
                    return "".to_string();
                }
            }
        }
        Err(e) => {
            println!("Failed to fetch html: {:?}", e);
            return "".to_string();
        }
    }
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