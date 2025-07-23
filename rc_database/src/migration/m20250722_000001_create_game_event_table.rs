use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250722_000001_create_game_event_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Permissions table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::game_event::Entity)
                    .col(
                        ColumnDef::new(crate::schema::game_event::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::game_event::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::Map).string().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::Mode).string().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::Visibility).string().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::AutoHeal).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::Start).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::End).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::game_event::Column::Variant).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Permissions table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::game_event::Entity).to_owned())
            .await
    }
}
