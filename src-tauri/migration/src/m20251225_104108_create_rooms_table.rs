use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Room::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Room::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Room::Start).big_integer().not_null())
                    .col(ColumnDef::new(Room::End).big_integer().not_null())
                    .col(ColumnDef::new(Room::Creator).uuid().not_null())
                    .col(
                        ColumnDef::new(Room::Members)
                            .json_binary()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(Room::Admins)
                            .json_binary()
                            .not_null()
                            .default("[]"),
                    )
                    .col(ColumnDef::new(Room::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Room::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Room {
    Table,
    Id,
    Title,
    Start,
    End,
    Members,
    Creator,
    Admins,
    CreatedAt,
    UpdatedAt,
}
