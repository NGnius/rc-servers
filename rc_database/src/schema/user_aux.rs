use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users_aux")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub descriptor: Descriptor,
    pub data: String, // usually JSON
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
    UserXP, // u32
    PremiumExpiry, // u64, seconds since Unix epoch
    UnlockedParts, // oj_rc_core::persist::user::UnlockedParts
    TechPoints, // u32
    UserRank, // u32
    UserFreeCurrency, // u64
    UserPaidCurrency, // u64
    GarageSlotOrder, // Vec<u32>, CSV
    LastSeen, // u64, seconds since Unix epoch
    SubscribedChannels, // Vec<String>, JSON
    AvatarId, // u32, u32::MAX means custom avatar
    RedeemedPromoCodes, // Vec<String>, JSON
}
