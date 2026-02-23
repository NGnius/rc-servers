use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "clans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub name: String,
    pub description: String,
    pub variant: ClanType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::clan_member::Entity")]
    ClanMember,
}

impl Related<super::clan_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClanMember.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "PascalCase")]
pub enum ClanType {
    Public,
    Private,
    Banned,
    Abandoned,
}
