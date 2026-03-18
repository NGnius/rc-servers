use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260317_000001_add_game_overrides"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Add game overrides columns
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::multiplayer_game::Entity)
                    .add_column(ColumnDef::new(crate::schema::multiplayer_game::Column::Overrides).string().not_null().default("".to_owned()))
                    .to_owned()
            )
            .await
    }

    // Define how to rollback this migration: Drop the added column
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::multiplayer_game::Entity)
                    .drop_column(crate::schema::multiplayer_game::Column::Overrides)
                    .to_owned()
            )
            .await
    }
}
