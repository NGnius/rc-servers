#[async_trait::async_trait]
pub trait FakeUser: Send + Sync {
    async fn on_init(&self, descriptors: &[oj_rc_core::persist::user::PlayerDescriptor], player_id: oj_rc_core::types::PlayerId);

    async fn on_ready(&self, real_players: &std::collections::HashMap<oj_rc_core::types::PlayerId, crate::matches::generic::UserSender>);

    //fn on_damage(&self, data: &rlnl::events::ingame::DestroyCubesFull);

    async fn on_end(&self);

    /// player id which is running the fake user (client AI only)
    async fn running_on(&self) -> Option<oj_rc_core::types::PlayerId> {
        None
    }
}
