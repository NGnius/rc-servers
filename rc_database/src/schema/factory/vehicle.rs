use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "factory_vehicles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub garage_id: i32,
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
    pub cube_data: Vec<u8>,
    pub colour_data: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::user::Entity",
        from = "Column::UserId",
        to = "super::super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::super::garage::Entity",
        from = "Column::UserId",
        to = "super::super::garage::Column::Id"
    )]
    Garage,
}

impl Related<super::super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::super::garage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
