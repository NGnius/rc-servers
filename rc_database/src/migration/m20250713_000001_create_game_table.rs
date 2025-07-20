use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250713_000001_create_game_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Permissions table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::multiplayer_game::Entity)
                    .col(
                        ColumnDef::new(crate::schema::multiplayer_game::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::Guid).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::Map).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::Mode).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::Visibility).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::AutoHeal).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::Variant).string().not_null())
                    .col(ColumnDef::new(crate::schema::multiplayer_game::Column::IsComplete).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Permissions table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::multiplayer_game::Entity).to_owned())
            .await
    }
}
