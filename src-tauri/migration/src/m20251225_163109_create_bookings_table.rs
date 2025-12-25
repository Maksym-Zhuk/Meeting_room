use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bookings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Bookings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Bookings::RoomId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::Title).string().not_null())
                    .col(ColumnDef::new(Bookings::CreatedBy).uuid().not_null())
                    .col(ColumnDef::new(Bookings::StartAt).big_integer().not_null())
                    .col(ColumnDef::new(Bookings::EndAt).big_integer().not_null())
                    .col(ColumnDef::new(Bookings::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Bookings::UpdatedAt).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bookings::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Bookings {
    Table,
    Id,
    RoomId,
    Title,
    CreatedBy,
    StartAt,
    EndAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Rooms {
    Table,
    Id,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
