use sea_orm::FromQueryResult;

#[derive(FromQueryResult)]
pub struct Id {
    pub id: i32,
}

#[derive(FromQueryResult)]
pub struct IdAndThumbnailVersion {
    pub id: i32,
    pub thumbnail_version: i32,
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
