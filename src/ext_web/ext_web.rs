use entity::{enums::Site, page};

use crate::scrapper::scrapper::Scrapper;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};

use super::jfl::{self};
use entity::page::Entity as Page;

pub struct ExtWeb {
    db: DatabaseConnection,
    sites: Vec<Site>,
}

impl ExtWeb {
    pub fn new(db: DatabaseConnection) -> ExtWeb {
        ExtWeb {
            db,
            sites: vec![Site::JFL],
        }
    }

    pub async fn sync(&self) {
        for site in &self.sites {
            match site {
                Site::JFL => jfl::sync_locations(&self.db).await,
            };
        }
    }
}
