use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pages::Title).string().null())
                    .col(ColumnDef::new(Pages::Site).string().not_null())
                    .col(ColumnDef::new(Pages::PageType).string().not_null())
                    .col(ColumnDef::new(Pages::Location).string().not_null())
                    .col(ColumnDef::new(Pages::Source).string().null())
                    .col(ColumnDef::new(Pages::IsApproved).boolean().not_null())
                    .col(ColumnDef::new(Pages::IsIgnored).boolean().not_null())
                    .col(ColumnDef::new(Pages::MagazineYear).integer().null())
                    .col(ColumnDef::new(Pages::MagazineMonth).integer().null())
                    .col(ColumnDef::new(Pages::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Pages::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Pages::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Pages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pages {
    Table,
    Id,
    Site,
    PageType,
    Title,
    Location,
    Source,
    IsApproved,
    IsIgnored,
    MagazineYear,
    MagazineMonth,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
