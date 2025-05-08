use sea_orm::FromQueryResult;

#[derive(FromQueryResult)]
pub struct Id {
    pub id: u32,
}
