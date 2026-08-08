// If you lose connection in edit mode and then open chat, you will lose all keyboard input
// until you receive a party or game invite
// https://git.ngram.ca/OpenJam/rc-servers/issues/127
//
// This implementation sends a game invite since it's on the same server

use crate::custom_game_tracker::{SessionInfo, UserInfo};

const WORKAROUND_SESSION_ID: &str = "sys_0_cg|workaround";
const WORKAROUND_PUBLIC_ID: &str = "MrWorkaround66 sys";

#[repr(u8)]
#[derive(Clone, Copy)]
enum InviteState {
    Invited,
    Accepted,
}

impl InviteState {
    #[inline]
    fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::Invited,
            1 => Self::Accepted,
            _ => panic!("Invalid InviteState {}", num),
        }
    }

    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct EditModeInputLockupWorkaround {
    active_users: tokio::sync::RwLock<std::collections::HashMap<i32, std::sync::atomic::AtomicU8>>,
    default_config_map: std::collections::HashMap<String, String>,
    default_config_core: oj_rc_core::persist::user::intercom::IntercomLobbyCustomGameConfig,
}

impl EditModeInputLockupWorkaround {
    pub fn new() -> Self {
        Self {
            active_users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            default_config_map: crate::custom_game_tracker::game_config_default_map(),
            default_config_core: crate::custom_game_tracker::game_config_default_core(),
        }
    }

    pub async fn add_user(&self, id: i32, public_id: String) -> SessionInfo {
        self.active_users.write().await.insert(id, std::sync::atomic::AtomicU8::new(InviteState::Invited.to_u8()));
        SessionInfo {
            session_id: WORKAROUND_SESSION_ID.to_owned(),
            config: self.default_config_map.clone(),
            config_core: self.default_config_core.clone(),
            users: vec![
                UserInfo {
                    public_id: WORKAROUND_PUBLIC_ID.to_owned(),
                    is_invited: false,
                    team: 1,
                    state: crate::data::custom_games::PlayerSessionStatus::Ready,
                },
                UserInfo {
                    public_id,
                    is_invited: true,
                    team: 0,
                    state: crate::data::custom_games::PlayerSessionStatus::Ready,
                }
            ],
        }
    }

    pub async fn get_user(&self, id: i32, public_id: &str) -> Option<SessionInfo> {
        if let Some(state) = self.active_users.read().await.get(&id) {
            let state = InviteState::from_u8(state.load(std::sync::atomic::Ordering::Relaxed));
            let is_invited = matches!(state, InviteState::Invited);
            Some(SessionInfo {
                session_id: WORKAROUND_SESSION_ID.to_owned(),
                config: self.default_config_map.clone(),
                config_core: self.default_config_core.clone(),
                users: vec![
                    UserInfo {
                        public_id: WORKAROUND_PUBLIC_ID.to_owned(),
                        is_invited: false,
                        team: 1,
                        state: crate::data::custom_games::PlayerSessionStatus::Ready,
                    },
                    UserInfo {
                        public_id: public_id.to_owned(),
                        is_invited,
                        team: 0,
                        state: crate::data::custom_games::PlayerSessionStatus::Ready,
                    }
                ],
            })
        } else {
            None
        }
    }

    pub async fn accept_invite(&self, id: i32) {
        let lock = self.active_users.read().await;
        if let Some(state) = lock.get(&id) {
            state.store(InviteState::Accepted.to_u8(), std::sync::atomic::Ordering::Relaxed);
        } else {
            log::warn!("Tried to use EditModeInputLockupWorkaround for non-added user {}", id);
        }
    }

    pub async fn remove_user(&self, id: i32) {
        self.active_users.write().await.remove(&id);
    }
}
