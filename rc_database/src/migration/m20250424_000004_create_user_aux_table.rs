use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250424_000004_create_user_aux_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User auxiliary table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::user_aux::Entity)
                    .col(
                        ColumnDef::new(crate::schema::user_aux::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::user_aux::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_aux-user_id")
                            .from(crate::schema::user_aux::Entity, crate::schema::user_aux::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::user_aux::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::user_aux::Column::Descriptor).string().not_null())
                    .col(ColumnDef::new(crate::schema::user_aux::Column::Data).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the User auxiliary table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::user_aux::Entity).to_owned())
            .await
    }
}
