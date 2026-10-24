use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261024_000001_create_game_vehicle_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Game Garages table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::multiplayer_game_vehicle::Entity)
                    .col(
                        ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_garages-user_id")
                            .from(crate::schema::multiplayer_game_vehicle::Entity, crate::schema::multiplayer_game_vehicle::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::GameId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_garages-game_id")
                            .from(crate::schema::multiplayer_game_vehicle::Entity, crate::schema::multiplayer_game_vehicle::Column::GameId)
                            .to(crate::schema::multiplayer_game::Entity, crate::schema::multiplayer_game::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::PlayerId).small_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::Tier).integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::Name).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::Uuid).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::Cpu).integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::MasteryLevel).integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::DeathAnimationId).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::SpawnAnimationId).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::WeaponOrder).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::RobotData).blob().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_vehicle::Column::ColourData).blob().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Game Garages table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::multiplayer_game_vehicle::Entity).to_owned())
            .await
    }
}
