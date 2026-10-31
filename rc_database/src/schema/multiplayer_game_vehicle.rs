use sea_orm::entity::prelude::*;

/// A vehicle in a multiplayer game.
/// This is useless once the associated multiplayer game has completed.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "game_garages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: Option<i32>,
    pub game_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub player_id: i16, // actually u8
    pub tier: i32,
    pub name: String,
    pub uuid: i64,
    pub cpu: i32,
    pub mastery_level: i32,
    pub death_animation_id: String,
    pub spawn_animation_id: String,
    pub weapon_order: String, // csv
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::multiplayer_game::Entity",
        from = "Column::GameId",
        to = "super::multiplayer_game::Column::Id"
    )]
    Game,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::multiplayer_game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
