use sea_orm::FromQueryResult;

#[derive(FromQueryResult)]
pub struct Id {
    pub id: u32,
}

#[derive(FromQueryResult)]
pub struct SingleColumn<T: sea_orm::TryGetable> {
    pub column: T,
}

impl <T: sea_orm::TryGetable> std::ops::Deref for SingleColumn<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.column
    }
}
