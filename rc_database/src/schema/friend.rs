use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "friends")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub friend_source: i32,
    pub friend_target: i32,
    pub state: FriendStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FriendSource",
        to = "super::user::Column::Id"
    )]
    Source,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FriendTarget",
        to = "super::user::Column::Id"
    )]
    Target,
}

/*impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Source.def()
    }
}*/

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Target.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "PascalCase")]
pub enum FriendStatus {
    InviteSent,
    InvitePending,
    Accepted,
    Declined,
    Cancelled,
    Removed,
}

pub const FINAL_STATUSES: [FriendStatus; 3] = [
    FriendStatus::Declined,
    FriendStatus::Cancelled,
    FriendStatus::Removed,
];
