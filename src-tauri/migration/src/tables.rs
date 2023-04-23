use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Paper {
    Table,
    Id,
    Title,
    Authors,
    Publication,
    PublicationDate,
    DOI,
    URL,
    FilePath,
    Abstract,
}
