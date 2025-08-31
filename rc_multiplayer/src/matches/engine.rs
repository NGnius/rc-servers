pub struct RlnlPacket {
    pub event: rlnl::event_code::NetworkEvent,
    pub property: literustlib::packet::Property,
    pub data: Box<dyn crate::Broadcastable>,
}

/// Functions returning a boolean indicate if the generic engine should also perform the default behaviour
/// (true = yes, do default behaviour)
#[async_trait::async_trait]
pub trait CustomGameLogic: Sized + Send + Sync + 'static {
    async fn on_player_join(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection, others: &[oj_rc_core::persist::user::PlayerDescriptor]) -> bool;
    async fn on_player_end(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> bool;
    async fn on_vehicle_destroyed(&self, generic: &super::GenericGamemodeEngine<Self>, killer: u8, victim: u8) -> bool;
    async fn on_vehicle_self_destruct(&self, generic: &super::GenericGamemodeEngine<Self>, user: u8, is_classic: bool) -> bool;
    async fn extra_sync_events(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> Vec<RlnlPacket>;
    async fn on_countdown_start(&self, generic: &super::GenericGamemodeEngine<Self>, game_start: chrono::DateTime<chrono::Utc>) -> bool;
    async fn on_game_completed(&self, generic: &super::GenericGamemodeEngine<Self>) -> bool;
    async fn on_broadcast(&self, generic: &super::GenericGamemodeEngine<Self>, user_id: i32, event_out: rlnl::event_code::NetworkEvent, event_in: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &Option<Box<dyn crate::Broadcastable>>, skip_user: bool) -> bool;
    async fn on_motion(&self, generic: &super::GenericGamemodeEngine<Self>, motion: &rlnl::machine_motion::MachineMotion, location: (f32, f32, f32)) -> bool;
    async fn on_custom(&self, generic: &super::GenericGamemodeEngine<Self>, user_id: i32, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: Box<dyn crate::Broadcastable>);
}

