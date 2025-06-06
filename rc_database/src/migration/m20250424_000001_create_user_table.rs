use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250424_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Users table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::user::Entity)
                    .col(
                        ColumnDef::new(crate::schema::user::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::user::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::user::Column::PublicId).string().not_null())
                    .col(ColumnDef::new(crate::schema::user::Column::DisplayName).string().not_null())
                    .col(ColumnDef::new(crate::schema::user::Column::Password).string().not_null())
                    .col(ColumnDef::new(crate::schema::user::Column::Email).string().not_null())
                    .col(ColumnDef::new(crate::schema::user::Column::SteamId).string())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Users table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::user::Entity).to_owned())
            .await
    }
}
