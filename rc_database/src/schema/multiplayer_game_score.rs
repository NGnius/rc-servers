use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "scores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub player_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub is_claimed: bool,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub heal_assists: i32,
    pub healed: i32,
    pub received_healed: i32,
    pub damaged: i32,
    pub received_damaged: i32,
    pub crystals: i32,
    pub total: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::multiplayer_game_player::Entity",
        from = "Column::PlayerId",
        to = "super::multiplayer_game_player::Column::Id"
    )]
    Player,
}

impl Related<super::multiplayer_game_player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
