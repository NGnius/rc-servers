use crate::matches::CustomGameLogic;

#[allow(dead_code)]
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

    async fn on_vehicle_self_destruct(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user: u8, _is_classic: bool) -> bool {
        true
    }

    async fn extra_sync_events(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> Vec<crate::matches::RlnlPacket> {
        Vec::default()
    }

    async fn on_countdown_start(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _game_start: chrono::DateTime<chrono::Utc>) -> bool {
        true
    }

    async fn on_game_completed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        true
    }

    async fn on_broadcast(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event_out: rlnl::event_code::NetworkEvent, _event_in: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: &Option<Box<dyn crate::Broadcastable>>, _skip_user: bool) -> bool {
        true
    }

    async fn on_motion(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _motion: &rlnl::machine_motion::MachineMotion, _location: (f32, f32, f32)) -> bool {
        true
    }

    async fn on_custom(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, _data: Box<dyn crate::Broadcastable>) {}
}
