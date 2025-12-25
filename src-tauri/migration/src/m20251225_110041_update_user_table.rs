use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::CreatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .add_column(
                        ColumnDef::new(User::UpdatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::CreatedAt)
                    .drop_column(User::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    CreatedAt,
    UpdatedAt,
}
