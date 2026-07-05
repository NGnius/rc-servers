use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260705_000001_create_federation_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Federation table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::federation::Entity)
                    .col(
                        ColumnDef::new(crate::schema::federation::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::federation::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::federation::Column::LastUsedTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::federation::Column::Domain).string().not_null())
                    .col(ColumnDef::new(crate::schema::federation::Column::Auth).string().not_null())
                    .col(ColumnDef::new(crate::schema::federation::Column::Cdn).string().not_null())
                    .col(ColumnDef::new(crate::schema::federation::Column::Factory).string().not_null())
                    .col(ColumnDef::new(crate::schema::federation::Column::Society).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Federation table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::federation::Entity).to_owned())
            .await
    }
}
