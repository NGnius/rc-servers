use sea_orm_migration::{MigratorTrait, MigrationTrait, prelude::async_trait};

mod m20250424_000001_create_user_table;
mod m20250424_000002_create_user_permissions_table;
mod m20250424_000003_create_garage_table;
mod m20250424_000004_create_user_aux_table;
mod m20250424_000005_create_campaign_tables;
mod m20250526_000001_add_garage_customisation;
mod m20250529_000001_create_sanction_table;
mod m20250713_000001_create_game_table;
mod m20250713_000002_create_player_table;
mod m20250722_000001_create_game_event_table;
mod m20250816_000001_add_fake_players;
mod m20250918_000001_add_player_variant;
mod m20251228_000001_create_score_table;
#[cfg(feature = "factory")]
mod m20260126_000001_create_factory_vehicle_table;
mod m20260215_000001_create_friend_table;

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
            Box::new(m20250529_000001_create_sanction_table::Migration),
            Box::new(m20250713_000001_create_game_table::Migration),
            Box::new(m20250713_000002_create_player_table::Migration),
            Box::new(m20250722_000001_create_game_event_table::Migration),
            Box::new(m20250816_000001_add_fake_players::Migration),
            Box::new(m20250918_000001_add_player_variant::Migration),
            Box::new(m20251228_000001_create_score_table::Migration),
            #[cfg(feature = "factory")]
            Box::new(m20260126_000001_create_factory_vehicle_table::Migration),
            Box::new(m20260215_000001_create_friend_table::Migration),
        ]
    }
}
