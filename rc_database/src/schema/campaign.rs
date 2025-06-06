use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "campaigns")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub campaign_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::campaign_difficulty_completion::Entity")]
    CampaignCompletion,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::campaign_difficulty_completion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CampaignCompletion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
