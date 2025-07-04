pub struct User {
    state: tokio::sync::RwLock<UserState>,
}

impl User {
    pub fn new() -> Self {
        Self {
            state: tokio::sync::RwLock::new(UserState::Connecting),
        }
    }

    pub async fn authenticate(&self, info: rlnl::events::loading::GameGuidInfo) -> bool {
        *self.state.write().await = UserState::Authenticated(UserInfo {
            game: info.game_guid.0,
            username: info.player_name.0,
        });
        true
    }
}

enum UserState {
    Connecting,
    Authenticated(UserInfo),
}

pub struct UserInfo {
    pub game: String,
    pub username: String,
}
