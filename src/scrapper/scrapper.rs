use sqlx::{Pool, Postgres};


pub trait Scrapper {
    fn sync_locations(&self, pool: &Pool<Postgres>) -> Vec<String>;
}
