use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookingMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BookingMembers::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BookingMembers::BookingId).uuid().not_null())
                    .col(ColumnDef::new(BookingMembers::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(BookingMembers::Table, BookingMembers::BookingId)
                            .to(Bookings::Table, Bookings::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BookingMembers::Table, BookingMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-booking-user-unique")
                            .col(BookingMembers::BookingId)
                            .col(BookingMembers::UserId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookingMembers::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum BookingMembers {
    Table,
    Id,
    BookingId,
    UserId,
}

#[derive(Iden)]
enum Bookings {
    Table,
    Id,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
