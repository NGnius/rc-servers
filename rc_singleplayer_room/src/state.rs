use std::sync::RwLock;

use tokio::sync::mpsc::UnboundedSender;
use polariton_server::ToSend;

#[derive(Debug, Default)]
pub struct UserAuthInfo {
    pub uuid: String,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub struct UserState {
    pub auth: RwLock<UserAuthInfo>,
    pub event_tx: UnboundedSender<ToSend>,
}

impl UserState {
    pub fn update_with_auth(&self, auth_str: &str) -> bool {
        let splits: Vec<&str> = auth_str.split(';').collect();
        if splits.len() != 3 {
            log::warn!("Invalid auth payload: {}", auth_str);
            false
        } else {
            let mut lock = self.auth.write().unwrap();
            lock.uuid = splits[0].to_owned();
            lock.token = splits[1].to_owned();
            lock.refresh_token = splits[2].to_owned();
            true
        }
    }

    pub fn new(event_tx: UnboundedSender<ToSend>) -> crate::UserTy {
        UserState {
            auth: RwLock::new(Default::default()),
            event_tx,
        }
    }
}
