use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250713_000002_create_player_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Permissions table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::multiplayer_game_player::Entity)
                    .col(
                        ColumnDef::new(crate::schema::multiplayer_game_player::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-players-user_id")
                            .from(crate::schema::multiplayer_game_player::Entity, crate::schema::multiplayer_game_player::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::GameId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-players-game_id")
                            .from(crate::schema::multiplayer_game_player::Entity, crate::schema::multiplayer_game_player::Column::GameId)
                            .to(crate::schema::multiplayer_game::Entity, crate::schema::multiplayer_game::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::PlayerId).small_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::Team).integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::Group).integer().null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::IsClaimed).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Permissions table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::multiplayer_game_player::Entity).to_owned())
            .await
    }
}
