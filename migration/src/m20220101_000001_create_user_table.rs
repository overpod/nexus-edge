use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(text(Users::Id).unique_key().primary_key())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp())
                    .col(text(Users::Login).unique_key())
                    .col(text(Users::Password))
                    .col(text(Users::Role))
                    .col(text(Users::Language))
                    .col(text(Users::ColorScheme))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Login,
    Password,
    Role,
    Language,
    ColorScheme,
}
