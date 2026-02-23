use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260221_000002_create_clan_member_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Clan Members table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::clan_member::Entity)
                    .col(
                        ColumnDef::new(crate::schema::clan_member::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::clan_member::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::clan_member::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clan_members-user_id")
                            .from(crate::schema::clan_member::Entity, crate::schema::clan_member::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::clan_member::Column::ClanId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clan_members-clan_id")
                            .from(crate::schema::clan_member::Entity, crate::schema::clan_member::Column::ClanId)
                            .to(crate::schema::clan::Entity, crate::schema::clan::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::clan_member::Column::Rank).string().not_null())
                    .col(ColumnDef::new(crate::schema::clan_member::Column::Status).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Clan Members table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::clan_member::Entity).to_owned())
            .await
    }
}
