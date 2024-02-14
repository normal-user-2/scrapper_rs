use sea_orm::entity::prelude::*;

use crate::enums::{PageType, Site};

// use super::models::{PageType, Site};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub site: Site,
    // used only for searching
    pub title: Option<String>,
    pub location: String,
    pub source: Option<String>,
    pub page_type: PageType,
    pub is_approved: bool,
    pub is_ignored: bool,
    pub magazine_year: Option<i32>,
    pub magazine_month: Option<i32>,
    #[sea_orm(ignore)]
    pub post_title: Option<String>,
    // #[sea_orm(ignore)]
    // pub approved_post: Option<Post>,
    // #[sea_orm(ignore)]
    // pub proposed_post: Option<Post>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
