#[async_trait::async_trait]
pub trait FakeUser: Send + Sync {
    async fn on_init(&self, descriptors: &Vec<oj_rc_core::persist::user::PlayerDescriptor>, player_id: u8);
    async fn on_ready(&self, real_players: &std::collections::HashMap<u8, crate::matches::generic::UserSender>);
    //fn on_damage(&self, data: &rlnl::events::ingame::DestroyCubesFull);
    async fn on_end(&self);
}
