use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260221_000001_create_clan_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Clans table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::clan::Entity)
                    .col(
                        ColumnDef::new(crate::schema::clan::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::clan::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::clan::Column::Name).string().not_null())
                    .col(ColumnDef::new(crate::schema::clan::Column::Description).string().not_null())
                    .col(ColumnDef::new(crate::schema::clan::Column::Variant).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Clans table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::clan::Entity).to_owned())
            .await
    }
}
