use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250526_000001_add_garage_customisation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Add death and spawn animation columns
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // sqlite adapter doesn't support multiple alter operations in one declaration
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::garage::Entity)
                    .add_column(ColumnDef::new(crate::schema::garage::Column::SpawnAnimationId).string().not_null().default("Spawn".to_owned()))
                    .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::garage::Entity)
                    .add_column(ColumnDef::new(crate::schema::garage::Column::DeathAnimationId).string().not_null().default("Explosion".to_owned()))
                    .to_owned()
            )
            .await

    }

    // Define how to rollback this migration: Drop the added colums.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::garage::Entity)
                    .drop_column(crate::schema::garage::Column::SpawnAnimationId)
                    .to_owned()
             )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::garage::Entity)
                    .drop_column(crate::schema::garage::Column::DeathAnimationId)
                    .to_owned()
            )
            .await
    }
}
