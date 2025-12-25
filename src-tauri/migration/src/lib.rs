pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20251225_162920_create_organizations_table;
mod m20251225_163020_create_organization_members_table;
mod m20251225_163045_create_rooms_table;
mod m20251225_163109_create_bookings_table;
mod m20251225_163141_create_booking_members_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20251225_162920_create_organizations_table::Migration),
            Box::new(m20251225_163020_create_organization_members_table::Migration),
            Box::new(m20251225_163045_create_rooms_table::Migration),
            Box::new(m20251225_163109_create_bookings_table::Migration),
            Box::new(m20251225_163141_create_booking_members_table::Migration),
        ]
    }
}
