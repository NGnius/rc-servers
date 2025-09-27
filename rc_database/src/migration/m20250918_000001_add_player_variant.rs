use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250918_000001_add_player_variant"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Add player variant columns
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::multiplayer_game_player::Entity)
                    .add_column(ColumnDef::new(crate::schema::multiplayer_game_player::Column::Variant).string().not_null().default(crate::schema::multiplayer_game_player::ClientType::Client))
                    .to_owned()
            )
            .await
    }

    // Define how to rollback this migration: Drop the added column
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::multiplayer_game_player::Entity)
                    .drop_column(crate::schema::multiplayer_game_player::Column::Variant)
                    .to_owned()
            )
            .await
    }
}
