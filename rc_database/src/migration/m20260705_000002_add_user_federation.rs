use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260705_000002_add_user_federation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Add user federation id column
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support multiple alter options in one operation
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::user::Entity)
                    .add_column(ColumnDef::new(crate::schema::user::Column::FederationId).integer().null())
                    .to_owned()
            )
            .await?;
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            manager
                .alter_table(
                    Table::alter()
                        .table(crate::schema::user::Entity)
                        .add_foreign_key(
                            ForeignKey::create()
                                .name("fk-user_federation_id")
                                .from(crate::schema::user::Entity, crate::schema::user::Column::FederationId)
                                .to(crate::schema::federation::Entity, crate::schema::federation::Column::Id)
                                .get_foreign_key(),
                        )
                        .to_owned()
                )
                .await
        } else {
            // SQLite doesn't support altering foreign keys of an existing table
            Ok(())
        }

    }

    // Define how to rollback this migration: Drop the added column
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support multiple alter options in one operation
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            // SQLite doesn't support altering foreign keys of an existing table
            manager
                .alter_table(
                    Table::alter()
                        .table(crate::schema::user::Entity)
                        .drop_foreign_key("fk-user_federation_id")
                        .to_owned()
                )
                .await?;
        }
        manager
            .alter_table(
                Table::alter()
                    .table(crate::schema::user::Entity)
                    .drop_column(crate::schema::user::Column::FederationId)
                    .to_owned()
            )
            .await
    }
}
