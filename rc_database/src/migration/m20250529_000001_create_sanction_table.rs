use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250529_000001_create_sanction_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Sanctions table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::sanction::Entity)
                    .col(
                        ColumnDef::new(crate::schema::sanction::Column::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::sanction::Column::UserId).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sanction-user_id")
                            .from(crate::schema::sanction::Entity, crate::schema::sanction::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::sanction::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::sanction::Column::IssuerId).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sanction-issuer_id")
                            .from(crate::schema::sanction::Entity, crate::schema::sanction::Column::IssuerId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::sanction::Column::IssuerName).string().not_null())
                    .col(ColumnDef::new(crate::schema::sanction::Column::Descriptor).string().not_null())
                    .col(ColumnDef::new(crate::schema::sanction::Column::Reason).string().not_null())
                    .col(ColumnDef::new(crate::schema::sanction::Column::Duration).big_integer())
                    .col(ColumnDef::new(crate::schema::sanction::Column::Acknowledged).big_integer())
                    .col(ColumnDef::new(crate::schema::sanction::Column::AppealerId).unsigned())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sanction-appealer_id")
                            .from(crate::schema::sanction::Entity, crate::schema::sanction::Column::AppealerId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::sanction::Column::AppealTime).big_integer())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Sanctions table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::sanction::Entity).to_owned())
            .await
    }
}
