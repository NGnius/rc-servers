use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250424_000005_create_campaign_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Campaigns table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::campaign::Entity)
                    .col(
                        ColumnDef::new(crate::schema::campaign::Column::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::campaign::Column::UserId).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-campaign-user_id")
                            .from(crate::schema::campaign::Entity, crate::schema::campaign::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::campaign::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::campaign::Column::CampaignId).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::campaign_difficulty_completion::Entity)
                    .col(
                        ColumnDef::new(crate::schema::campaign_difficulty_completion::Column::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::campaign_difficulty_completion::Column::CampaignId).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-campaigns_completion-campaign_id")
                            .from(crate::schema::campaign_difficulty_completion::Entity, crate::schema::campaign_difficulty_completion::Column::CampaignId)
                            .to(crate::schema::campaign::Entity, crate::schema::campaign::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::campaign_difficulty_completion::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::campaign_difficulty_completion::Column::Level).unsigned().not_null())
                    .col(ColumnDef::new(crate::schema::campaign_difficulty_completion::Column::Wave).unsigned().not_null())
                    .col(ColumnDef::new(crate::schema::campaign_difficulty_completion::Column::Complete).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Campaigns table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::campaign::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(crate::schema::campaign_difficulty_completion::Entity).to_owned())
            .await
    }
}
