use super::tables::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Paper::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Paper::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Paper::Title).string().not_null())
                    .col(ColumnDef::new(Paper::Authors).string().not_null())
                    .col(ColumnDef::new(Paper::Publication).string().not_null())
                    .col(ColumnDef::new(Paper::PublicationDate).date().not_null())
                    .col(ColumnDef::new(Paper::DOI).string().null())
                    .col(ColumnDef::new(Paper::URL).string().null())
                    .col(ColumnDef::new(Paper::FilePath).string().not_null())
                    .col(ColumnDef::new(Paper::Abstract).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Paper::Table).to_owned())
            .await
    }
}
