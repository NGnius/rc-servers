use sea_orm_migration::{MigratorTrait, MigrationTrait, prelude::async_trait};

mod m20250424_000001_create_user_table;
mod m20250424_000002_create_user_permissions_table;
mod m20250424_000003_create_garage_table;
mod m20250424_000004_create_user_aux_table;
mod m20250424_000005_create_campaign_tables;
mod m20250526_000001_add_garage_customisation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250424_000001_create_user_table::Migration),
            Box::new(m20250424_000002_create_user_permissions_table::Migration),
            Box::new(m20250424_000003_create_garage_table::Migration),
            Box::new(m20250424_000004_create_user_aux_table::Migration),
            Box::new(m20250424_000005_create_campaign_tables::Migration),
            Box::new(m20250526_000001_add_garage_customisation::Migration),
        ]
    }
}
