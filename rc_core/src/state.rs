use crate::persist::user::UserProvider;
use polariton_server::ToSend;

pub struct UserState<C: Clone = ()> {
    state: std::sync::RwLock<InitState<C>>,
    event_tx: tokio::sync::mpsc::UnboundedSender<ToSend<C>>,
}

impl <C: Clone> UserState<C> {
    pub fn update_with_auth(&self, auth_str: &str) -> bool {
        self.update_with_auth_ext(auth_str, |_| Some(Default::default()))
    }

    pub fn update_with_auth_ext<F: FnOnce(&crate::persist::user::UserToken) -> Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>>>(&self, auth_str: &str, ext_f: F) -> bool {
        let mut lock = self.state.write().unwrap();
        match &*lock {
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
                    let ext = if let Some(ext) = ext_f(&token) {
                        ext
                    } else {
                        return false;
                    };
                    match auth.authenticate(token, ext) {
                        Ok(user) => {
                            *lock = InitState::Authenticated(std::sync::Arc::new(user));
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

    pub fn new(provider: std::sync::Arc<crate::persist::user::UserImpl>, event_tx: tokio::sync::mpsc::UnboundedSender<ToSend<C>>) -> Self {
        Self {
            state: std::sync::RwLock::new(InitState::Unauthenticated(provider)),
            event_tx,
        }
    }

    pub fn user(&self) -> Result<std::sync::Arc<Box<dyn crate::persist::user::User<C> + Send + Sync>>, i16> {
        let lock = self.state.read().unwrap();
        match &*lock {
            InitState::Unauthenticated(_) => Err(120),
            InitState::Authenticated(user) => Ok(user.clone()),
        }
    }

    pub fn event(&self, event_data: ToSend<C>) {
        self.event_tx.send(event_data).unwrap()
    }

    pub fn event_sender(&self) -> tokio::sync::mpsc::UnboundedSender<ToSend<C>> {
        self.event_tx.clone()
    }
}

enum InitState<C> {
    Unauthenticated(std::sync::Arc<crate::persist::user::UserImpl>),
    Authenticated(std::sync::Arc<Box<dyn crate::persist::user::User<C> + Send + Sync>>),
}
