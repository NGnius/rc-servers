pub struct RlnlPacket {
    pub event: rlnl::event_code::NetworkEvent,
    pub property: literustlib::packet::Property,
    pub data: Box<dyn crate::Broadcastable>,
}

/// Functions returning a boolean indicate if the generic engine should also perform the default behaviour
/// (true = yes, do default behaviour)
#[async_trait::async_trait]
pub trait CustomGameLogic: Sized + Send + Sync + 'static {
    /// Called when player joins the game server (after authentication).
    async fn on_player_join(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection, others: &[oj_rc_core::persist::user::PlayerDescriptor]) -> bool;
    /// Called when player leaves the game server (i.e. player disconnects).
    async fn on_player_end(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> bool;
    /// Called when a player's vehicle is destroyed by another player.
    async fn on_vehicle_destroyed(&self, generic: &super::GenericGamemodeEngine<Self>, killer: u8, victim: u8) -> bool;
    /// Called when a player's vehicle self-destructs.
    async fn on_vehicle_self_destruct(&self, generic: &super::GenericGamemodeEngine<Self>, user: u8, is_classic: bool) -> bool;
    /// Called when a kill bonus event is received after a confirmed kill
    async fn on_kill_bonus(&self, generic: &super::GenericGamemodeEngine<Self>, killer: u8, victim: u8) -> bool;
    /// Called during loading, before the sync stage is entered.
    ///
    /// Roughly, loading is as follows: `Loading -> WaitingForSync -> Sync -> WaitingToStart -> InGame`.
    async fn extra_sync_events(&self, generic: &super::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> Vec<RlnlPacket>;
    /// Called when loading completes and the countdown is starting
    async fn on_countdown_start(&self, generic: &super::GenericGamemodeEngine<Self>, game_start: chrono::DateTime<chrono::Utc>) -> bool;
    /// Called when the game is marked as complete
    async fn on_game_completed(&self, generic: &super::GenericGamemodeEngine<Self>) -> bool;
    /// Called when various network events are broadcast from one client but before they are sent to the rest of the clients
    #[allow(clippy::too_many_arguments)]
    async fn on_broadcast(&self, generic: &super::GenericGamemodeEngine<Self>, user_id: i32, event_out: rlnl::event_code::NetworkEvent, event_in: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &Option<Box<dyn crate::Broadcastable>>, skip_user: bool) -> bool;
    /// Called when a vehicle motion event is received from a client
    async fn on_motion(&self, generic: &super::GenericGamemodeEngine<Self>, motion: &rlnl::machine_motion::MachineMotion, location: (f32, f32, f32)) -> bool;
    /// Called when a gamemode-specific network event is received from a client
    async fn on_custom(&self, generic: &super::GenericGamemodeEngine<Self>, user_id: i32, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: Box<dyn crate::Broadcastable>);

    /// Called when an enemy vehicle is spotted
    async fn on_spot_vehicle(&self, generic: &super::GenericGamemodeEngine<Self>, user_id: i32, remote_player: u8) -> bool;
}

