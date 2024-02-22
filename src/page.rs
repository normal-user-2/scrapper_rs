use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct Page {
    pub id: i64,
    pub site: String,
    // used only for searching
    pub title: Option<String>,
    pub location: String,
    pub source: Option<String>,
    pub page_type: String,
    pub is_approved: bool,
    pub is_ignored: bool,
    pub magazine_year: Option<i32>,
    pub magazine_month: Option<i32>,
    // ignore sqlx
    // #[sea_orm(ignore)]
    // pub approved_post: Option<Post>,
    // #[sea_orm(ignore)]
    // pub proposed_post: Option<Post>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
