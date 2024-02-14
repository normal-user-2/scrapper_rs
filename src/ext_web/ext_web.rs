use entity::enums::Site;

use sea_orm::DatabaseConnection;

use super::jfl::{self};

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
