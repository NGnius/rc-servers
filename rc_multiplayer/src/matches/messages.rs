pub enum TimeoutVariant {
    WaitingForLoadingSync,
    WaitingForGameStart,
}

pub enum GameMessage {
    NewConnection {
        user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
        game_guid: String,
        connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
        response: tokio::sync::oneshot::Sender<Option<ErrorMessage>>,
        sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>,
    },
    EndConnection {
        user_id: i32,
        is_unregister: bool,
    },
    RequestLeave {
        user_id: i32,
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
    SelfDestruct {
        user_id: i32,
        is_classic: bool,
    },
    FlippingStarted { // flip yeah!
        user_id: i32,
    },
    MapPing {
        user_id: i32,
        ping: rlnl::events::ingame::MapPing,
    },
    KillBonus {
        user_id: i32,
        shootee: u8,
        shooter: u8,
    },
    AssistBonus {
        user_id: i32,
        shootee: u8,
        shooters: Vec<u8>,
    },
    HealAssistBonus {
        user_id: i32,
        healer: u8,
        healee: u8,
    },
    DestroyCubesBonus {
        user_id: i32,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus,
    },
    HealCubesBonus {
        user_id: i32,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus,
    },
    BroadcastRlnl {
        user_id: i32,
        event: rlnl::event_code::NetworkEvent,
        event_in: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn crate::Broadcastable>>,
    },
    RebroadcastRlnl {
        skip_user_id: i32,
        event: rlnl::event_code::NetworkEvent,
        event_in: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn crate::Broadcastable>>,
    },
    CustomLogicRlnl {
        user_id: i32,
        event: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Box<dyn crate::Broadcastable>,
    },
    PlayerInputChanged {
        user_id: i32,
        data: rlnl::events::ingame::PlayerIdAndInputData,
    },
    Motion {
        user_id: i32,
        motion: rlnl::machine_motion::MachineMotion,
    },
    NoOp,
    LoadingTimeout {
        timeout: TimeoutVariant,
        response: tokio::sync::oneshot::Sender<bool>,
    },
}

impl GameMessage {
    pub fn user_id(&self) -> i32 {
        match self {
            Self::NewConnection { user, .. } => {
                user.account_id()
            }
            Self::EndConnection { user_id, .. } => *user_id,
            Self::RequestLeave { user_id, .. } => *user_id,
            Self::LoadingProgress { user_id, .. } => *user_id,
            Self::RequestLoadingProgress { user_id, .. } => *user_id,
            Self::WeaponSelect { user_id, .. } => *user_id,
            Self::RequestLoadingSync { user_id, .. } => *user_id,
            Self::LoadComplete { user_id, .. } => *user_id,
            Self::SpotVehicle { user_id, .. } => *user_id,
            Self::DestroyVehicle { user_id, .. } => *user_id,
            Self::SelfDestruct { user_id, .. } => *user_id,
            Self::FlippingStarted { user_id, .. } => *user_id,
            Self::MapPing { user_id, .. } => *user_id,
            Self::KillBonus { user_id, .. } => *user_id,
            Self::AssistBonus { user_id, .. } => *user_id,
            Self::HealAssistBonus { user_id, .. } => *user_id,
            Self::DestroyCubesBonus { user_id, .. } => *user_id,
            Self::HealCubesBonus { user_id, .. } => *user_id,
            Self::BroadcastRlnl { user_id, .. } => *user_id,
            Self::RebroadcastRlnl { skip_user_id, .. } => *skip_user_id,
            Self::CustomLogicRlnl { user_id, .. } => *user_id,
            Self::PlayerInputChanged { user_id, .. } => *user_id,
            Self::Motion { user_id, .. } => *user_id,
            Self::NoOp => unreachable!("NoOp is irrelevant for user ID"),
            Self::LoadingTimeout { .. } => unreachable!("Timeout is irrelevant for user ID"),
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
