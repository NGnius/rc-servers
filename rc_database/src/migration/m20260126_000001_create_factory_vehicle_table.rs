use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260126_000001_create_factory_vehicle_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Factory Vehicles table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(crate::schema::factory::vehicle::Entity)
                    .col(
                        ColumnDef::new(crate::schema::factory::vehicle::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-factory_vehicles-user_id")
                            .from(crate::schema::factory::vehicle::Entity, crate::schema::factory::vehicle::Column::UserId)
                            .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::GarageId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-factory_vehicles-garage_id")
                            .from(crate::schema::factory::vehicle::Entity, crate::schema::factory::vehicle::Column::GarageId)
                            .to(crate::schema::garage::Entity, crate::schema::garage::Column::Id),
                    )
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::CreationTime).big_integer().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::Name).string().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::Description).string().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::AddedTime).date_time().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::ExpiryTime).date_time().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::Cpu).integer().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::TotalRobotRanking).integer().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::GetCount).integer().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::Buyable).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::RemovedDate).date_time().null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::BanDate).date_time().null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::Featured).boolean().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::BannerMessage).string().null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::CombatRating).double().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::CosmeticRating).double().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::CubeAmounts).string().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::CubeData).blob().not_null())
                    .col(ColumnDef::new(crate::schema::factory::vehicle::Column::ColourData).blob().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Factory Vehicles table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(crate::schema::factory::vehicle::Entity).to_owned())
            .await
    }
}
