use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250424_000002_create_user_permissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Permissions table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::permissions::Entity)
                    .col(
                        ColumnDef::new(crate::schema::permissions::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::permissions::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-permissions-user_id")
                            .from(crate::schema::permissions::Entity, crate::schema::permissions::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::permissions::Column::Moderator).boolean().not_null())
                    //.col(crate::schema::permissions::Column::Moderator.def()) // I wish this worked...
                    .col(ColumnDef::new(crate::schema::permissions::Column::Administrator).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::permissions::Column::Developer).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::permissions::Column::Royalty).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::permissions::Column::Banned).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Permissions table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::permissions::Entity).to_owned())
            .await
    }
}
