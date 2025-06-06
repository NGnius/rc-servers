use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250424_000003_create_garage_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Garages table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::garage::Entity)
                    .col(
                        ColumnDef::new(crate::schema::garage::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::garage::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-garage-user_id")
                            .from(crate::schema::garage::Entity, crate::schema::garage::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::garage::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::Slot).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::Name).string().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::CrfId).integer())
                    .col(ColumnDef::new(crate::schema::garage::Column::WasRated).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::MovementCategories).string().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::Uuid).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::ThumbnailVersion).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::TotalRobotCpu).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::TotalCosmeticCpu).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::TotalRobotRanking).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::BayCpu).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::TutorialRobot).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::StarterRobotIndex).integer())
                    .col(ColumnDef::new(crate::schema::garage::Column::ControlType).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::VerticalStrafing).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::SidewaysDriving).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::TracksTurnOnSpot).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::MasteryLevel).integer().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::BaySkinId).string().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::WeaponOrder).string().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::RobotData).blob().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::ColourData).blob().not_null())
                    .col(ColumnDef::new(crate::schema::garage::Column::Selected).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Garages table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::garage::Entity).to_owned())
            .await
    }
}
