use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rooms::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Rooms::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Rooms::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Rooms::Name).string().not_null())
                    .col(ColumnDef::new(Rooms::CreatedBy).uuid().not_null())
                    .col(ColumnDef::new(Rooms::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Rooms::UpdatedAt).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Rooms::Table, Rooms::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Rooms::Table, Rooms::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rooms::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Rooms {
    Table,
    Id,
    OrganizationId,
    Name,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Organizations {
    Table,
    Id,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
