use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "clan_members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub user_id: i32,
    pub clan_id: i32,
    pub rank: ClanMemberRank,
    pub status: ClanMemberStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::clan::Entity",
        from = "Column::ClanId",
        to = "super::clan::Column::Id"
    )]
    Clan,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::clan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clan.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "PascalCase")]
pub enum ClanMemberRank {
    Member,
    Officer,
    Leader,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "PascalCase")]
pub enum ClanMemberStatus {
    Invited,
    Confirmed,
    Deactivated,
}
