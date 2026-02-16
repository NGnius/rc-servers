use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub public_id: String,
    pub display_name: String,
    pub password: String,
    pub email: String,
    pub steam_id: Option<String>, // u64
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::permissions::Entity")]
    Permission,
    #[sea_orm(has_many = "super::garage::Entity")]
    Garages,
    #[sea_orm(has_many = "super::user_aux::Entity")]
    Aux,
    #[sea_orm(has_many = "super::campaign::Entity")]
    Campaigns,
    #[sea_orm(has_many = "super::multiplayer_game_player::Entity")]
    Player,
    #[sea_orm(has_many = "super::factory::vehicle::Entity")]
    FactoryUploads,
    #[sea_orm(has_many = "super::friend::Entity")]
    Friends, // this will probably join the wrong column (i.e. in the wrong direction)
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl Related<super::garage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Garages.def()
    }
}

impl Related<super::user_aux::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Aux.def()
    }
}

impl Related<super::campaign::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Campaigns.def()
    }
}

impl Related<super::multiplayer_game_player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl Related<super::factory::vehicle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FactoryUploads.def()
    }
}

impl Related<super::friend::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Friends.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
