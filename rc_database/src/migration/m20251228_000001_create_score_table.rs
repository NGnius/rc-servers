use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251228_000001_create_score_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Scores table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::multiplayer_game_score::Entity)
                    .col(
                        ColumnDef::new(crate::schema::multiplayer_game_score::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::PlayerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-scores-player_id")
                            .from(crate::schema::multiplayer_game_score::Entity, crate::schema::multiplayer_game_score::Column::PlayerId)
                            .to(crate::schema::multiplayer_game_player::Entity, crate::schema::multiplayer_game_player::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::IsClaimed).boolean().default(false))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Kills).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Deaths).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Assists).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::HealAssists).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Healed).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::ReceivedHealed).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Damaged).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::ReceivedDamaged).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Crystals).integer().default(0))
                    .col(ColumnDef::new(crate::schema::multiplayer_game_score::Column::Total).integer().default(0))
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Scores table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::multiplayer_game_score::Entity).to_owned())
            .await
    }
}
