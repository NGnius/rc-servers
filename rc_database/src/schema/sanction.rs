use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sanctions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub user_id: u32,
    pub creation_time: i64, // seconds since unix epoch
    pub issuer_id: u32,
    pub issuer_name: String,
    pub descriptor: Descriptor,
    pub reason: String,
    pub duration: Option<i64>, // seconds after creation_time (null means permanent or irrelevant)
    pub acknowledged: Option<i64>, // seconds since unix epoch
    pub appealer_id: Option<u32>, // moderator who approved appeal
    pub appeal_time: Option<i64>, // seconds since unix epoch
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "PascalCase")]
pub enum Descriptor {
    Warn,
    Mute,
    Ban,
    Note,
    Kick,
}
