use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "garages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub user_id: u32,
    pub creation_time: i64, // seconds since unix epoch
    pub slot: u32,
    pub name: String,
    pub crf_id: Option<u32>,
    pub was_rated: bool,
    pub movement_categories: String, // csv?
    pub uuid: i64,
    pub thumbnail_version: u32,
    pub total_robot_cpu: u32,
    pub total_cosmetic_cpu: u32,
    pub total_robot_ranking: u32,
    pub bay_cpu: u32,
    pub tutorial_robot: bool,
    pub starter_robot_index: Option<u32>,
    pub control_type: ControlType,
    pub vertical_strafing: bool,
    pub sideways_driving: bool,
    pub tracks_turn_on_spot: bool,
    pub mastery_level: u32,
    pub bay_skin_id: String,
    pub weapon_order: String, // csv?
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub selected: bool,
}

impl Model {
    pub fn cube_count(&self) -> u32 {
        if self.robot_data.len() >= 4 {
            u32::from_le_bytes([self.robot_data[0], self.robot_data[1], self.robot_data[2], self.robot_data[3]])
        } else {
            0
        }
    }
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

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "TinyInteger")]
pub enum ControlType {
    Camera = 0,
    Keyboard = 1,
    Count = 2,
}
