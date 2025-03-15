use crate::persist::user::UserProvider;

pub struct UserState<C: Clone> {
    state: InitState<C>,
}

impl <C: Clone> UserState<C> {
    pub fn update_with_auth(&mut self, auth_str: &str) -> bool {
        match &self.state {
            InitState::Unauthenticated(auth) => {
                let splits: Vec<&str> = auth_str.split(';').collect();
                if splits.len() != 3 {
                    log::warn!("Invalid auth payload: {}", auth_str);
                    false
                } else {
                    let token = crate::persist::user::UserToken {
                        uuid: splits[0].to_owned(),
                        token: splits[1].to_owned(),
                        refresh_token: splits[2].to_owned(),
                    };
                    match auth.authenticate(token) {
                        Ok(user) => {
                            self.state = InitState::Authenticated(user);
                            true
                        },
                        Err(e) => {
                            log::error!("Failed to authenticate {}: {}", splits[0], e);
                            false
                        }
                    }
                }
            },
            InitState::Authenticated(_) => {
                log::warn!("User was already authenticated, ignoring");
                true
            }
        }

    }

    pub fn new(provider: std::sync::Arc<crate::persist::user::UserImpl>) -> Self {
        Self {
            state: InitState::Unauthenticated(provider),
        }
    }

    pub fn user(&self) -> Result<&dyn crate::persist::user::User<C>, i16> {
        match &self.state {
            InitState::Unauthenticated(_) => Err(120),
            InitState::Authenticated(user) => Ok(user.as_ref()),
        }
    }
}

enum InitState<C> {
    Unauthenticated(std::sync::Arc<crate::persist::user::UserImpl>),
    Authenticated(Box<dyn crate::persist::user::User<C> + Send + Sync>),
}
