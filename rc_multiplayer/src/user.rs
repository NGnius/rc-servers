pub struct User {
    state: tokio::sync::RwLock<UserState>,
}

impl User {
    pub fn new(provider: std::sync::Arc<oj_rc_core::persist::user::UserImpl>) -> Self {
        Self {
            state: tokio::sync::RwLock::new(UserState::Unauthenticated(provider)),
        }
    }

    pub async fn authenticate(&self, info: rlnl::events::loading::GameGuidInfo) -> bool {
        let init_state_clone = self.state.read().await.clone();
        match init_state_clone {
            UserState::Unauthenticated(auth) => {
                let result = <oj_rc_core::persist::user::UserImpl as oj_rc_core::UserProvider<()>>::multiplayer_authenticate::<'_, '_>(&auth, info.player_name.0.clone()).await;
                match result {
                    Ok(user) => {
                        *self.state.write().await = UserState::Authenticated(UserInfo {
                            game: info.game_guid.0,
                            user: std::sync::Arc::new(user),
                        });
                        true
                    },
                    Err(e) => {
                        log::error!("Failed to authenticate {}: {}", info.player_name.0, e.message);
                        false
                    }
                }
            },
            UserState::Authenticated(info) => {
                log::warn!("User {} already authenticated, ignoring", info.user.account_id());
                true
            }
        }
    }

    pub async fn user(&self) -> Option<std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync>>> {
        match &*self.state.read().await {
            UserState::Unauthenticated(_) => None,
            UserState::Authenticated(user) => Some(user.user.clone()),
        }
    }

    pub async fn game_guid(&self) -> Option<String> {
        match &*self.state.read().await {
            UserState::Unauthenticated(_) => None,
            UserState::Authenticated(user) => Some(user.game.clone()),
        }
    }
}

#[derive(Clone)]
enum UserState {
    Unauthenticated(std::sync::Arc<oj_rc_core::persist::user::UserImpl>),
    Authenticated(UserInfo),
}

#[derive(Clone)]
pub struct UserInfo {
    pub game: String,
    pub user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync>>,
}
