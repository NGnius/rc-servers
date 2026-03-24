pub trait TeamSelector: crate::Plugin {
    fn select_team(&self, game: &str, index: usize, user_id: Option<i32>, group: Option<String>) -> u8;
}
