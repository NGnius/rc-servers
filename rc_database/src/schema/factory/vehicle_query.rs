use sea_orm::{DerivePartialModel, FromQueryResult};
use sea_orm::entity::prelude::DateTime;

#[derive(FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "crate::schema::factory::vehicle::Entity")]
pub struct VehicleInfo {
    pub id: i32,
    pub user_id: i32,
    pub creation_time: i64, // seconds since unix epoch
    pub name: String,
    pub description: String,
    pub added_time: DateTime,
    pub expiry_time: DateTime,
    pub cpu: i32,
    pub total_robot_ranking: i32,
    pub get_count: i32,
    pub buyable: bool,
    pub removed_date: Option<DateTime>,
    pub ban_date: Option<DateTime>,
    pub featured: bool,
    pub banner_message: Option<String>,
    pub combat_rating: f64,
    pub cosmetic_rating: f64,
    pub cube_amounts: String,
}

#[derive(FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "crate::schema::user::Entity")]
pub struct VehicleOwnerInfo {
    pub public_id: String,
    pub display_name: String,
}
