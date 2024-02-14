pub use sea_orm_migration::prelude::*;

mod m20240214_090547_create_pages_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240214_090547_create_pages_table::Migration),
        ]
    }
}
