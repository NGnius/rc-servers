use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct UserState {
    pub uuid: String,
    pub token: String,
    pub refresh_token: String,
}

impl UserState {
    pub fn update_with_auth(&mut self, auth_str: &str) -> bool {
        let splits: Vec<&str> = auth_str.split(';').collect();
        if splits.len() != 3 {
            log::warn!("Invalid auth payload: {}", auth_str);
            false
        } else {
            self.uuid = splits[0].to_owned();
            self.token = splits[1].to_owned();
            self.refresh_token = splits[2].to_owned();
            true
        }
    }

    pub fn new() -> crate::UserTy {
        RwLock::new(UserState::default())
    }
}
