use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "game_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub map: String,
    pub mode: super::multiplayer_game::GameMode,
    pub visibility: super::multiplayer_game::MapVisibility,
    pub auto_heal: bool,
    pub start: i64, // seconds since unix epoch
    pub end: i64, // seconds since unix epoch
    pub variant: EventVariant,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "PascalCase")]
pub enum EventVariant {
    Multiplayer,
    Singleplayer,
}
