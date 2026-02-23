use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260215_000001_create_friend_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Friends table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::friend::Entity)
                    .col(
                        ColumnDef::new(crate::schema::friend::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::friend::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::friend::Column::FriendSource).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-friends-friend_source")
                            .from(crate::schema::friend::Entity, crate::schema::friend::Column::FriendSource)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::friend::Column::FriendTarget).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-friends-friend_target")
                            .from(crate::schema::friend::Entity, crate::schema::friend::Column::FriendTarget)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::friend::Column::State).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Friends table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::friend::Entity).to_owned())
            .await
    }
}
