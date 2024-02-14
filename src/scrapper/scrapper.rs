use sea_orm::DatabaseConnection;

pub trait Scrapper {
    fn sync_locations(&self, db: &DatabaseConnection) -> Vec<String>;
}
