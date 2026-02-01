use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "garages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub slot: i32,
    pub name: String,
    pub crf_id: Option<i32>,
    pub was_rated: bool,
    pub movement_categories: String, // csv?
    pub uuid: i64,
    pub thumbnail_version: i32,
    pub total_robot_cpu: i32,
    pub total_cosmetic_cpu: i32,
    pub total_robot_ranking: i32,
    pub bay_cpu: i32,
    pub tutorial_robot: bool,
    pub starter_robot_index: Option<i32>,
    pub control_type: ControlType,
    // control options
    pub vertical_strafing: bool,
    pub sideways_driving: bool,
    pub tracks_turn_on_spot: bool,
    // end control options
    pub mastery_level: i32,
    pub bay_skin_id: String,
    pub death_animation_id: String,
    pub spawn_animation_id: String,
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
    #[sea_orm(has_many = "super::factory::vehicle::Entity")]
    FactoryUploads,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::factory::vehicle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FactoryUploads.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ControlType {
    Camera = 0,
    Keyboard = 1,
    Count = 2,
}
