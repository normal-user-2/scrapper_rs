// use entity::enums::Site;

use std::sync::Arc;

use crate::site::JFL;
// use sea_orm::{ActiveModelTrait, DatabaseConnection, QueryOrder};
use sqlx::{pool::PoolConnection, Pool, Postgres};

use crate::ext_web::helper;

use super::jfl::{self};
use chrono::Utc;
// use entity::page;
// use entity::page::Entity as Page;
// use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

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

    pub async fn sync(&self) {
        for site in &self.sites {
            // location
            self.sync_locations(site).await;

            // source
            // self.sync_source(site).await;
        }
    }

    async fn sync_locations(&self, site: &str) {
        println!("Syncing locations for site: {:?}", site);

        match site {
            JFL => jfl::sync_locations(&self.pool).await,
            _ => {
                println!("Site not found: {:?}", site);
            }
        };
    }

    // async fn sync_source(&self, site: &Site) {
    //     println!("{:?} syncing sources", site);

    //     let pages = Page::find()
    //         .order_by_desc(page::Column::Id)
    //         .filter(Condition::all().add(page::Column::Site.eq(site.clone())))
    //         .all(&self.pool)
    //         .await;

    //     if pages.is_err() {
    //         println!("{:?} failed to fetch pages: {:?}", site, pages.err());
    //         return;
    //     }

    //     let pages = pages.unwrap();

    //     for page in pages {
    //         let loc = page.location.clone();

    //         let source = helper::get_url_body(&loc).await;

    //         if source == "" {
    //             println!("{:?} failed to fetch source for page: {:?}", site, loc);
    //             continue;
    //         }

    //         if let Some(existing_source) = &page.source {
    //             if existing_source.eq(&source) {
    //                 continue;
    //             }
    //         }

    //         println!("{:?}, new source found for page: {:?}", site, loc);

    //         let mut title: String;
    //         match site {
    //             Site::JFL => {
    //                 title = jfl::extract_title(&source).await;
    //             }
    //         }
    //         title = helper::normalize_title(title);

    //         // update source and page

    //         let current_time = Utc::now().naive_utc();

    //         let mut page: page::ActiveModel = page.into();
    //         page.source = Set(Some(source));
    //         page.title = Set(Some(title));

    //         page.updated_at = Set(current_time);

    //         let updated_page = page.update(&self.pool).await;

    //         match updated_page {
    //             Ok(_) => {
    //                 continue;
    //             }
    //             Err(e) => {
    //                 println!("{:?} failed to update page: {:?}", site, e);
    //             }
    //         }
    //     }
    // }
}
