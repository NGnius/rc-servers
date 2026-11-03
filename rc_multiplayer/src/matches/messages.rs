pub enum TimeoutVariant {
    WaitingForLoadingSync,
    WaitingForGameStart,
}

pub enum GameMessage {
    NewConnection {
        user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
        game_guid: oj_rc_core::types::GameGuid,
        connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
        response: tokio::sync::oneshot::Sender<Option<ErrorMessage>>,
        sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>,
    },
    EndConnection {
        user_id: oj_rc_core::types::UserId,
        is_unregister: bool,
    },
    RequestLeave {
        user_id: oj_rc_core::types::UserId,
    },
    LoadingProgress {
        user_id: oj_rc_core::types::UserId,
        user_name: String,
        progress: f32,
    },
    RequestLoadingProgress {
        user_id: oj_rc_core::types::UserId,
    },
    WeaponSelect {
        user_id: oj_rc_core::types::UserId,
        machine_id: oj_rc_core::types::PlayerId,
        category: oj_rc_core::data::weapon_list::ItemCategory,
        size: oj_rc_core::data::cube_list::ItemTier,
    },
    RequestLoadingSync {
        user_id: oj_rc_core::types::UserId,
    },
    LoadComplete {
        user_id: oj_rc_core::types::UserId,
    },
    SpotVehicle {
        user_id: oj_rc_core::types::UserId,
        remote_player: oj_rc_core::types::PlayerId,
    },
    DestroyVehicle {
        user_id: oj_rc_core::types::UserId,
        remote_player: oj_rc_core::types::PlayerId,
        killer_player: oj_rc_core::types::PlayerId,
    },
    SelfDestruct {
        user_id: oj_rc_core::types::UserId,
        is_classic: bool,
    },
    FlippingStarted { // flip yeah!
        user_id: oj_rc_core::types::UserId,
    },
    MapPing {
        user_id: oj_rc_core::types::UserId,
        ping: rlnl::events::ingame::MapPing,
    },
    KillBonus {
        user_id: oj_rc_core::types::UserId,
        shootee: oj_rc_core::types::PlayerId,
        shooter: oj_rc_core::types::PlayerId,
    },
    AssistBonus {
        user_id: oj_rc_core::types::UserId,
        shootee: oj_rc_core::types::PlayerId,
        shooters: Vec<oj_rc_core::types::PlayerId>,
    },
    HealAssistBonus {
        user_id: oj_rc_core::types::UserId,
        healer: oj_rc_core::types::PlayerId,
        healee: oj_rc_core::types::PlayerId,
    },
    DestroyCubesBonus {
        user_id: oj_rc_core::types::UserId,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus,
    },
    HealCubesBonus {
        user_id: oj_rc_core::types::UserId,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus,
    },
    BroadcastRlnl {
        user_id: oj_rc_core::types::UserId,
        event: rlnl::event_code::NetworkEvent,
        event_in: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn crate::Broadcastable>>,
    },
    RebroadcastRlnl {
        skip_user_id: oj_rc_core::types::UserId,
        event: rlnl::event_code::NetworkEvent,
        event_in: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn crate::Broadcastable>>,
    },
    CustomLogicRlnl {
        user_id: oj_rc_core::types::UserId,
        event: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Box<dyn crate::Broadcastable>,
    },
    PlayerInputChanged {
        user_id: oj_rc_core::types::UserId,
        data: rlnl::events::ingame::PlayerIdAndInputData,
    },
    Motion {
        user_id: oj_rc_core::types::UserId,
        motion: rlnl::machine_motion::MachineMotion,
    },
    NoOp,
    LoadingTimeout {
        timeout: TimeoutVariant,
        response: tokio::sync::oneshot::Sender<bool>,
    },
}

impl GameMessage {
    pub fn user_id(&self) -> oj_rc_core::types::UserId {
        match self {
            Self::NewConnection { user, .. } => {
                user.user_id()
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
