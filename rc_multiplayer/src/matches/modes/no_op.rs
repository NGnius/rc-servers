use crate::matches::CustomGameLogic;

pub struct NoOpLogic;

#[async_trait::async_trait]
impl CustomGameLogic for NoOpLogic {
    async fn on_player_join(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection, _others: &[oj_rc_core::persist::user::PlayerDescriptor]) -> bool {
        true
    }

    async fn on_player_end(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> bool {
        true
    }

    async fn on_vehicle_destroyed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _killer: u8, _victim: u8) -> bool {
        true
    }

    async fn extra_sync_events(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> Vec<crate::matches::RlnlPacket> {
        Vec::default()
    }
}
