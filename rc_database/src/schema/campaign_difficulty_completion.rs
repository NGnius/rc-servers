use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "campaigns_completion")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub campaign_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub level: i32,
    pub wave: i32,
    pub complete: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::campaign::Entity",
        from = "Column::CampaignId",
        to = "super::campaign::Column::Id"
    )]
    Campaign,
}

impl Related<super::campaign::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Campaign.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
