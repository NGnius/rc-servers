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
    async fn extra_sync_events(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> Vec<RlnlPacket>;
}

