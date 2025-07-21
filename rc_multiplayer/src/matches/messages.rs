pub enum GameMessage {
    NewConnection {
        user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
        game_guid: String,
        connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
        response: tokio::sync::oneshot::Sender<Option<ErrorMessage>>,
        sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>,
    },
    LoadingProgress {
        user_id: i32,
        user_name: String,
        progress: f32,
    },
    RequestLoadingProgress {
        user_id: i32,
    },
    WeaponSelect {
        user_id: i32,
        machine_id: u8,
        category: oj_rc_core::data::weapon_list::ItemCategory,
        size: oj_rc_core::data::cube_list::ItemTier,
    },
    RequestLoadingSync {
        user_id: i32,
    },
    LoadComplete {
        user_id: i32,
    },
    SpotVehicle {
        user_id: i32,
        remote_player: u8,
    },
    DestroyVehicle {
        user_id: i32,
        remote_player: u8,
        killer_player: u8,
    },
    BroadcastRlnl {
        user_id: i32,
        event: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn byteserde::ser_heap::ByteSerializeHeap + Send + Sync>>,
    },
    RebroadcastRlnl {
        skip_user_id: i32,
        event: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn byteserde::ser_heap::ByteSerializeHeap + Send + Sync>>,
    },
    Motion {
        user_id: i32,
        data: bytes::Bytes,
    },
    NoOp,
}

impl GameMessage {
    pub fn user_id(&self) -> i32 {
        match self {
            Self::NewConnection { user, .. } => {
                user.user_id()
            }
            Self::LoadingProgress { user_id, .. } => *user_id,
            Self::RequestLoadingProgress { user_id, .. } => *user_id,
            Self::WeaponSelect { user_id, .. } => *user_id,
            Self::RequestLoadingSync { user_id, .. } => *user_id,
            Self::LoadComplete { user_id, .. } => *user_id,
            Self::SpotVehicle { user_id, .. } => *user_id,
            Self::DestroyVehicle { user_id, .. } => *user_id,
            Self::BroadcastRlnl { user_id, .. } => *user_id,
            Self::RebroadcastRlnl { skip_user_id, .. } => *skip_user_id,
            Self::Motion { user_id, .. } => *user_id,
            Self::NoOp => unreachable!("NoOp is irrelevant for user ID"),
        }
    }
}

#[derive(Debug)]
pub struct ErrorMessage {
    pub message: String,
    pub inner: Option<Box<dyn std::error::Error + Send>>,
}

impl std::error::Error for ErrorMessage {}

impl core::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(inner) = &self.inner {
            write!(f, "game communication error: {}; {}", self.message, inner)
        } else {
            write!(f, "game communication error: {}", self.message)
        }
    }
}
