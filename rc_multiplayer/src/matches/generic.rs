pub(super) struct UserConnection {
    pub(super) user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
    pub(super) connection: UserSender,
    aliases: Vec<u8>,
}

pub(super) struct UserDescriptor {
    pub(super) state: std::sync::Arc<UserState>,
    pub(super) machine: MachineState,
    pub(super) descriptor: std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>,
    pub(super) counters: UserData,
}

impl UserDescriptor {
    fn new(descriptor: oj_rc_core::persist::user::PlayerDescriptor) -> Self {
        Self {
            state: std::sync::Arc::new(UserState::new()),
            machine: MachineState::new(),
            descriptor: std::sync::Arc::new(descriptor),
            counters: UserData::new(),
        }
    }
}

/*#[allow(dead_code)]
pub(super) struct FakeUser {
    pub(super) state: std::sync::Arc<UserState>,
    pub(super) machine: MachineState,
    pub(super) descriptor: oj_rc_core::persist::user::PlayerDescriptor,
    pub(super) counters: UserData,
}

impl FakeUser {
    fn new(descriptor: oj_rc_core::persist::user::PlayerDescriptor) -> Self {
        Self {
            state: std::sync::Arc::new(UserState {
                mode: std::sync::atomic::AtomicU8::new(ConnectionMode::InGame.to_u8()),
                progress: std::sync::atomic::AtomicU8::new(100),
            }),
            machine: MachineState::new(),
            descriptor,
            counters: UserData::new(),
        }
    }
}*/

#[derive(Clone)]
pub(super) struct UserSender {
    pub(super) connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
    pub(super) sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>,
}

impl UserSender {
    pub fn rlnl(&self) -> crate::handlers::RlnlSender<'_> {
        crate::handlers::RlnlSender::new(&self.sender)
    }
}

pub(super) struct UserState {
    pub(super) mode: std::sync::atomic::AtomicU8,
    pub(super) progress: std::sync::atomic::AtomicU8, // percent
}

impl UserState {
    fn new() -> Self {
        Self {
            mode: std::sync::atomic::AtomicU8::new(ConnectionMode::Loading.to_u8()),
            progress: std::sync::atomic::AtomicU8::new(0),
        }
    }
}

pub(super) struct MachineState {
    pub(super) selected_weapon: WeaponInfo,
    pub(super) location: Location,
    pub(super) is_alive: std::sync::Arc<std::sync::atomic::AtomicBool>
}

impl MachineState {
    fn new() -> Self {
        Self {
            selected_weapon: WeaponInfo::new(),
            location: Location::new(),
            is_alive: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }
}

pub(super) struct WeaponInfo {
    category: std::sync::atomic::AtomicU32,
    size: std::sync::atomic::AtomicU32,
}

impl WeaponInfo {
    fn new() -> Self {
        Self {
            category: std::sync::atomic::AtomicU32::new(0),
            size: std::sync::atomic::AtomicU32::new(0),
        }
    }
}

pub(super) struct Location {
    pub x: atomic_float::AtomicF32,
    pub y: atomic_float::AtomicF32,
    pub z: atomic_float::AtomicF32,
}

impl Location {
    fn new() -> Self {
        Self {
            x: atomic_float::AtomicF32::new(0.0),
            y: atomic_float::AtomicF32::new(0.0),
            z: atomic_float::AtomicF32::new(0.0),
        }
    }
}

pub(super) struct UserData {
    pub score_id: std::sync::atomic::AtomicI64,
    pub kills: std::sync::atomic::AtomicU32,
    pub deaths: std::sync::atomic::AtomicU32,
    pub assists: std::sync::atomic::AtomicU32,
    pub heal_assists: std::sync::atomic::AtomicU32,
    pub healed: std::sync::atomic::AtomicU32,
    pub received_healed: std::sync::atomic::AtomicU32,
    pub cubes: std::sync::atomic::AtomicU32,
    pub received_cubes: std::sync::atomic::AtomicU32, // damage taken
    //pub segments_captured: std::sync::atomic::AtomicU32, // TODO
    pub crystals: std::sync::atomic::AtomicU32, // crystals destroyed
}

impl UserData {
    fn new() -> Self {
        Self {
            score_id: std::sync::atomic::AtomicI64::new(i64::MIN),
            kills: std::sync::atomic::AtomicU32::new(0),
            deaths: std::sync::atomic::AtomicU32::new(0),
            assists: std::sync::atomic::AtomicU32::new(0),
            heal_assists: std::sync::atomic::AtomicU32::new(0),
            healed: std::sync::atomic::AtomicU32::new(0),
            received_healed: std::sync::atomic::AtomicU32::new(0),
            cubes: std::sync::atomic::AtomicU32::new(0),
            received_cubes: std::sync::atomic::AtomicU32::new(0),
            //segments_captured: std::sync::atomic::AtomicU32::new(0),
            crystals: std::sync::atomic::AtomicU32::new(0),
        }
    }

    pub(super) fn generic_score(&self) -> u32 {
        self.kills.load(std::sync::atomic::Ordering::Relaxed) * 1_000
        + self.assists.load(std::sync::atomic::Ordering::Relaxed) * 100
        //+ self.heal_assists.load(std::sync::atomic::Ordering::Relaxed) * 200
        + self.healed.load(std::sync::atomic::Ordering::Relaxed)
        + self.cubes.load(std::sync::atomic::Ordering::Relaxed)
        + self.crystals.load(std::sync::atomic::Ordering::Relaxed) * 25
    }

    pub(super) fn get_generic_packet(&self, player_id: u8, stat: rlnl::types::IngameStatId, delta: Option<u32>) -> rlnl::events::ingame::UpdateGameStats {
        let (stat_amount, backup_delta) = match stat {
            rlnl::types::IngameStatId::DestroyedCubes
            | rlnl::types::IngameStatId::DestroyedCubesInProtection
            | rlnl::types::IngameStatId::DestroyedCubesDefendingTheBase => (self.cubes.load(std::sync::atomic::Ordering::SeqCst), 1),
            rlnl::types::IngameStatId::Kill => (self.kills.load(std::sync::atomic::Ordering::Relaxed), 1_000),
            rlnl::types::IngameStatId::KillAssist => (self.assists.load(std::sync::atomic::Ordering::Relaxed), 100),
            rlnl::types::IngameStatId::HealCubes => (self.healed.load(std::sync::atomic::Ordering::SeqCst), 1),
            rlnl::types::IngameStatId::HealAssist => (self.heal_assists.load(std::sync::atomic::Ordering::SeqCst), 200),
            rlnl::types::IngameStatId::RobotDestroyed => (self.deaths.load(std::sync::atomic::Ordering::Relaxed), 0),
            rlnl::types::IngameStatId::DestroyedProtoniumCubes => (self.deaths.load(std::sync::atomic::Ordering::Relaxed), 25),
            //rlnl::types::IngameStatId::Points => (self.kills.load(std::sync::atomic::Ordering::Relaxed), 1),
            s => panic!("Cannot generate game stat {:?}", s)
        };
        rlnl::events::ingame::UpdateGameStats {
            player_id,
            stat_id: stat,
            amount: stat_amount,
            score: self.generic_score(),
            delta_score: delta.unwrap_or(backup_delta),
        }
    }

    fn as_core(&self) -> oj_rc_core::persist::user::PlayerScore {
        let id_maybe = self.score_id.load(std::sync::atomic::Ordering::Relaxed);
        oj_rc_core::persist::user::PlayerScore {
            id: id_maybe.try_into().ok(),
            kills: self.kills.load(std::sync::atomic::Ordering::Relaxed),
            deaths: self.deaths.load(std::sync::atomic::Ordering::Relaxed),
            assists: self.assists.load(std::sync::atomic::Ordering::Relaxed),
            heal_assists: self.heal_assists.load(std::sync::atomic::Ordering::Relaxed),
            healed: self.healed.load(std::sync::atomic::Ordering::Relaxed),
            received_healed: self.received_healed.load(std::sync::atomic::Ordering::Relaxed),
            damaged: self.cubes.load(std::sync::atomic::Ordering::Relaxed),
            received_damaged: self.received_cubes.load(std::sync::atomic::Ordering::Relaxed),
            crystals: self.crystals.load(std::sync::atomic::Ordering::Relaxed),
            total: self.generic_score(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(super) enum ConnectionMode {
    Loading = 0,
    WaitingForSync = 1,
    Sync = 2,
    WaitingToStart = 3,
    InGame = 4,
    Disconnected = 5,
}

impl ConnectionMode {
    #[inline]
    pub(super) fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::Loading,
            1 => Self::WaitingForSync,
            2 => Self::Sync,
            3 => Self::WaitingToStart,
            4 => Self::InGame,
            5 => Self::Disconnected,
            x => panic!("Unrecognized ConnectionMode {}", x),
        }
    }

    #[inline]
    pub(super) fn to_u8(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum LoadingState {
    Starting = 0,
    InSync = 1,
    InGame  = 2,
    End = 3,
}

impl LoadingState {
    #[inline]
    pub(super) fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::Starting,
            1 => Self::InSync,
            2 => Self::InGame,
            3 => Self::End,
            x => panic!("Unrecognized ConnectionMode {}", x),
        }
    }

    #[inline]
    pub(super) fn to_u8(self) -> u8 {
        self as u8
    }
}

struct UnclaimedStats {
    kills: tokio::sync::Mutex<std::collections::HashMap<KillAttribution, chrono::DateTime<chrono::Utc>>>,
}

impl UnclaimedStats {
    const DEBOUNCE_PERIOD: std::time::Duration = std::time::Duration::from_secs(2);
    fn new() -> Self {
        Self {
            kills: tokio::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    // returns true if is new
    async fn debounce_kill(&self, attr: KillAttribution) -> bool {
        let now = chrono::Utc::now();
        if let Some(time) = self.kills.lock().await.insert(attr, now) {
            (now - time).to_std().unwrap_or_default() > Self::DEBOUNCE_PERIOD
        } else {
            true
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct KillAttribution {
    killer: u8,
    victim: u8,
}

pub(super) struct GenericGamemodeEngine<L: super::CustomGameLogic> {
    pub users: tokio::sync::RwLock<std::collections::HashMap<u8, std::sync::Arc<UserConnection>>>,
    descriptors: std::collections::HashMap<u8, UserDescriptor>,
    user_id_map: std::collections::HashMap<i32, u8>,
    //pub recv: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<super::GameMessage>>,
    //pub send: tokio::sync::mpsc::Sender<super::GameMessage>,
    //pub game_guid: String,
    is_complete: std::sync::atomic::AtomicBool,
    pub game_start: std::sync::atomic::AtomicI64,
    pub map_config: std::sync::Arc<oj_rc_core::persist::config::MapConfig>,
    pub game_descriptor: oj_rc_core::persist::user::GameDescriptor,
    pub game_duration: std::time::Duration,
    //pub players_info: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>,
    pub custom_logic_handler: L,
    //pub fake_users: std::collections::HashMap<u8, FakeUser>,
    pub fakes_handler: super::fake::Handler,
    unclaimed: UnclaimedStats,
    loading_state: std::sync::Arc<std::sync::atomic::AtomicU8>,
    self_sender: Option<tokio::sync::mpsc::Sender<super::GameMessage>>,
    mp_config: std::sync::Arc<oj_rc_core::persist::config::MultiplayerSettings>,
}

impl <L: super::CustomGameLogic> GenericGamemodeEngine<L> {
    const END_OF_SYNC_DELAY: std::time::Duration = std::time::Duration::from_millis(100);
    const COUNTDOWN_DURATION: std::time::Duration = std::time::Duration::from_secs(5);

    pub fn new(
        super_conf: crate::matches::engine::SuperConfig,
        custom: L,
        fakes_handler: super::fake::Handler,
        mp_config: std::sync::Arc<oj_rc_core::persist::config::MultiplayerSettings>,
    ) -> Self {

        /*let fake_users = players.iter()
            .filter(|player| player.user_id.is_none())
            .map(|player| (player.team as u8, FakeUser::new(player.to_owned())))
            .collect();*/

        let descriptors = super_conf.players.iter()
            .map(|player| (player.player_id, UserDescriptor::new(player.to_owned())))
            .collect();

        let user_id_map = super_conf.players.iter()
            .filter_map(|player| player.user_id.map(|id| (id, player.player_id)))
            .collect();
        Self {
            users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            descriptors,
            user_id_map,
            is_complete: std::sync::atomic::AtomicBool::new(false),
            game_start: std::sync::atomic::AtomicI64::new(i64::MIN),
            map_config: std::sync::Arc::new(super_conf.map),
            game_descriptor: super_conf.descriptor,
            game_duration: std::time::Duration::from_secs((super_conf.game_mode.game_time_minutes as u64) * 60),
            custom_logic_handler: custom,
            fakes_handler,
            unclaimed: UnclaimedStats::new(),
            loading_state: std::sync::Arc::new(std::sync::atomic::AtomicU8::new(LoadingState::Starting.to_u8())),
            self_sender: None,
            mp_config,
        }
    }

    #[inline]
    pub(super) fn game_guid(&self) -> &'_ str {
        &self.game_descriptor.guid
    }

    pub(super) fn user_key_by_user_id(&self, user_id: i32) -> Option<u8> {
        self.user_id_map.get(&user_id).copied()
    }

    pub(super) fn user_descriptor(&self, player_id: u8) -> Option<&'_ UserDescriptor> {
        self.descriptors.get(&player_id)
    }

    pub(super) fn user_descriptors(&self) -> &'_ std::collections::HashMap<u8, UserDescriptor> {
        &self.descriptors
    }

    /*pub(super) async fn user_connection(&self, player_id: u8) -> Option<&'_ UserConnection> {
        self.users.read().await.get(&player_id)
    }*/

    pub(super) async fn rebroadcast<T: byteserde::ser_heap::ByteSerializeHeap + ?Sized>(&self, user_id: i32, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &T, in_game: bool) {
        for (player_id, conn) in self.users.read().await.iter() {
            if user_id == conn.user.account_id() { continue; }
            if conn.aliases.contains(player_id) { continue; }
            if in_game {
                let mode = ConnectionMode::from_u8(self.user_descriptor(*player_id).unwrap().state.mode.load(std::sync::atomic::Ordering::Relaxed));
                if !matches!(mode, ConnectionMode::InGame) { continue; }
            }
            let sender = crate::handlers::RlnlSender::new(&conn.connection.sender);
            crate::events::log_lnl_send_failure(sender.send_data(
                data,
                code,
                property,
                &conn.connection.connection,
            ).await);
        }
    }

    pub(super) async fn rebroadcast_dataless(&self, user_id: i32, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, in_game: bool) {
        for (player_id, conn) in self.users.read().await.iter() {
            if user_id == conn.user.account_id() { continue; }
            if conn.aliases.contains(player_id) { continue; }
            if in_game {
                let mode = ConnectionMode::from_u8(self.user_descriptor(*player_id).unwrap().state.mode.load(std::sync::atomic::Ordering::Relaxed));
                if !matches!(mode, ConnectionMode::InGame) { continue; }
            }
            let sender = crate::handlers::RlnlSender::new(&conn.connection.sender);
            crate::events::log_lnl_send_failure(sender.send_empty(
                code,
                property,
                &conn.connection.connection,
            ).await);
        }
    }

    pub(super) async fn broadcast<T: byteserde::ser_heap::ByteSerializeHeap + ?Sized>(&self, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &T, in_game: bool) {
        for (player_id, conn) in self.users.read().await.iter() {
            if conn.aliases.contains(player_id) { continue; }
            if in_game {
                let mode = ConnectionMode::from_u8(self.user_descriptor(*player_id).unwrap().state.mode.load(std::sync::atomic::Ordering::Relaxed));
                if !matches!(mode, ConnectionMode::InGame) { continue; }
            }
            let sender = conn.connection.rlnl();
            crate::events::log_lnl_send_failure(sender.send_data(
                data,
                code,
                property,
                &conn.connection.connection,
            ).await);
        }
    }

    pub(super) async fn broadcast_dataless(&self, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, in_game: bool) {
        for (player_id, conn) in self.users.read().await.iter() {
            if conn.aliases.contains(player_id) { continue; }
            if in_game {
                let mode = ConnectionMode::from_u8(self.user_descriptor(*player_id).unwrap().state.mode.load(std::sync::atomic::Ordering::Relaxed));
                if !matches!(mode, ConnectionMode::InGame) { continue; }
            }
            let sender = conn.connection.rlnl();
            crate::events::log_lnl_send_failure(sender.send_empty(
                code,
                property,
                &conn.connection.connection,
            ).await);
        }
    }

    pub(super) fn elapsed_game_time(&self) -> f32 {
        let game_start = self.game_start.load(std::sync::atomic::Ordering::Relaxed);
        //let game_end = (game_start as u64) + self.game_duration.as_secs();
        let duration = self.game_duration.as_secs();
        let now = chrono::Utc::now().timestamp();
        ((now - game_start) as f32) / duration as f32
    }

    /// seconds since Unix epoch
    pub(super) fn game_end(&self) -> i64 {
        self.game_start.load(std::sync::atomic::Ordering::Relaxed) + self.game_duration.as_secs() as i64
    }

    pub(super) fn is_game_past_end_time(&self) -> bool {
        let game_start = self.game_start.load(std::sync::atomic::Ordering::Relaxed);
        let game_end = self.game_end();
        game_start != i64::MIN && game_end <= chrono::Utc::now().timestamp()
    }

    pub(super) fn players_info(&self) -> Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>> {
        self.descriptors.values()
            .map(|p| p.descriptor.clone())
            .collect()
    }

    pub(super) fn real_player_count(&self) -> usize {
        self.descriptors.values()
            .filter(|p| p.descriptor.user_id.is_some())
            .count()
    }

    pub(super) async fn send_to_player<T: byteserde::ser_heap::ByteSerializeHeap + ?Sized>(&self, player_id: u8, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &T) {
        if let Some(player) = self.users.read().await.get(&player_id) {
            crate::events::log_lnl_send_failure(player.connection.rlnl().send_data(
                data,
                code,
                property,
                &player.connection.connection,
            ).await);
        }
    }

    pub(super) fn spawn(mut self) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(super::CHANNEL_BOUND);
        self.self_sender = Some(tx.clone());
        tokio::spawn(self.run(rx));
        tx
    }

    pub(super) async fn run(self, mut recv: tokio::sync::mpsc::Receiver<super::GameMessage>) {
        let mut is_engaged = true;
        while !recv.is_closed() && is_engaged {
            if let Some(msg) = recv.recv().await {
                match msg {
                    super::GameMessage::NewConnection { user, game_guid, connection, response, sender } => {
                        self.on_new_connection(user, game_guid, connection, response, sender).await;
                    },
                    super::GameMessage::EndConnection { user_id, is_unregister } => {
                        is_engaged = self.on_end_connection(user_id, is_unregister).await;
                    },
                    super::GameMessage::RequestLeave { user_id } => {
                        self.on_request_leave(user_id).await;
                    }
                    super::GameMessage::LoadingProgress { user_id, user_name, progress } => {
                        self.on_loading_progress(user_id, user_name, progress).await;
                    }
                    super::GameMessage::RequestLoadingProgress { user_id } => {
                        self.on_request_loading_progress(user_id).await;
                    },
                    super::GameMessage::WeaponSelect { user_id, machine_id, category, size } => {
                        self.on_weapon_select(user_id, machine_id, category, size).await;
                    },
                    super::GameMessage::RequestLoadingSync { user_id } => {
                        self.on_request_loading_sync(user_id).await;
                    },
                    super::GameMessage::LoadComplete { user_id } => {
                        self.on_load_complete(user_id).await;
                    },
                    super::GameMessage::SpotVehicle { user_id, remote_player } => {
                        self.on_spot_vehicle(user_id, remote_player).await;
                    },
                    super::GameMessage::DestroyVehicle { user_id, remote_player, killer_player } => {
                        self.on_destroy_vehicle(user_id, remote_player, killer_player).await;
                    },
                    super::GameMessage::SelfDestruct { user_id, is_classic } => {
                        self.on_self_destruct(user_id, is_classic).await;
                    },
                    super::GameMessage::FlippingStarted { user_id } => {
                        self.on_flipping_started(user_id).await;
                    },
                    super::GameMessage::MapPing { user_id, ping } => {
                        self.on_map_ping(user_id, ping).await;
                    },
                    super::GameMessage::KillBonus { user_id, shootee, shooter } => {
                        self.on_kill_bonus(user_id, shootee, shooter).await;
                    },
                    super::GameMessage::AssistBonus { user_id, shootee, shooters } => {
                        self.on_assist_bonus(user_id, shootee, shooters).await;
                    },
                    super::GameMessage::HealAssistBonus { user_id, healer, healee } => {
                        self.on_heal_assist_bonus(user_id, healer, healee).await;
                    },
                    super::GameMessage::DestroyCubesBonus { user_id, info } => {
                        self.on_destroy_cubes_bonus(user_id, info).await;
                    },
                    super::GameMessage::HealCubesBonus { user_id, info } => {
                        self.on_heal_cubes_bonus(user_id, info).await;
                    },
                    super::GameMessage::BroadcastRlnl { user_id, event, event_in, property, data } => {
                        self.on_broadcast(user_id, event, event_in, property, data, false).await;
                    },
                    super::GameMessage::RebroadcastRlnl { skip_user_id, event, event_in, property, data } => {
                        self.on_broadcast(skip_user_id, event, event_in, property, data, true).await;
                    },
                    super::GameMessage::CustomLogicRlnl { user_id, event, property, data } => {
                        self.custom_logic_handler.on_custom(&self, user_id, event, property, data).await;
                    },
                    super::GameMessage::PlayerInputChanged { user_id, data } => {
                        self.on_player_input_changed(user_id, data).await;
                    },
                    super::GameMessage::Motion { user_id, motion } => {
                        self.on_motion(user_id, motion).await;
                    },
                    super::GameMessage::NoOp => {},
                    super::GameMessage::LoadingTimeout { timeout, response } => {
                        self.on_timeout(timeout, response).await;
                    }
                }
            }
        }
        self.fakes_handler.stop();
        log::info!("Game {} has exited", self.game_guid());
    }

    async fn on_new_connection(&self,
        user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
        game_guid: String,
        connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
        response: tokio::sync::oneshot::Sender<Option<super::messages::ErrorMessage>>,
        sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>
    ) {
        if self.game_guid() != game_guid {
            log::error!("Game guid does not match (got: {}, expected: {})", game_guid, self.game_guid());
            response.send(Some(super::messages::ErrorMessage {
                message: format!("Game guid does not match (got: {}, expected: {})", game_guid, self.game_guid()),
                inner: None,
            })).unwrap_or_default();
        } else {
            //tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            //let id = users.len() as u8;
            let user_id = user.account_id();
            let player_info_opt = self.descriptors.values().find(|p| p.descriptor.user_id == Some(user_id));
            if player_info_opt.is_none() {
                log::warn!("User {} tried to connect to match {} which they are not in", user_id, self.game_guid());
                connection.goodbye(&sender).await;
                response.send(Some(super::messages::ErrorMessage {
                    message: format!("User {} not in match {})", user_id, self.game_guid()),
                    inner: None,
                })).unwrap_or_default();
                return;
            }
            let player_info = player_info_opt.unwrap();
            let id = player_info.descriptor.player_id;
            let aliases = self.fakes_handler.get_client_ais().await.into_iter().find(|(key, _val)| *key == id).map(|(_key, val)| val).unwrap_or_default();
            log::info!("AIs running on new player {}: {:?}", id, aliases);
            let new_user = UserConnection {
                user,
                connection: UserSender {
                    connection,
                    sender,
                },
                aliases,
            };
            if self.custom_logic_handler.on_player_join(self, &new_user, player_info).await {
                //self.spawn_send_loading_events(&new_user, id, self.players_info.clone());
                crate::events::log_lnl_send_failure(new_user.connection.rlnl().send_data(
                    &rlnl::events::ingame::PlayerId { player: id },
                    rlnl::event_code::NetworkEvent::GameGuidValidated,
                    literustlib::packet::Property::ReliableOrdered,
                    &new_user.connection.connection
                ).await);
                log::debug!("User {} is validated to play game {}", new_user.user.account_id(), game_guid);
                let new_user = std::sync::Arc::new(new_user);
                if let Err(e) = new_user.user.save_player_connected_status(self.game_guid(), true).await {
                    log::error!("Failed to mark player {} (user {}) as connected to game {}: {}", id, user_id, self.game_guid(), e);
                }
                let mut users = self.users.write().await;
                let was_empty = users.is_empty();
                users.insert(id, new_user.clone());
                for fake_id in new_user.aliases.iter() {
                    if let Some(player_desc) = self.user_descriptor(*fake_id) {
                        if self.custom_logic_handler.on_player_join(self, &new_user, player_desc).await {
                            users.insert(*fake_id, new_user.clone());
                        }
                    } else {
                        log::warn!("Non-existent fake player id {} was encountered while connecting, ignoring", *fake_id);
                    }
                }
                if was_empty {
                    self.start_loading_sync_timeouter().await;
                }
            }
            response.send(None).unwrap_or_default();
        }
    }

    async fn on_end_connection(&self, user_id: i32, is_unregister: bool) -> bool {
        if let Some(player_id) = self.user_key_by_user_id(user_id) {
            let conn_opt = self.users.write().await.remove(&player_id);
            if let Some(conn) = conn_opt {
                let user_info = self.user_descriptor(player_id).unwrap();
                if self.custom_logic_handler.on_player_end(self, &conn, user_info).await {
                    user_info.state.mode.store(ConnectionMode::Disconnected.to_u8(), std::sync::atomic::Ordering::Relaxed);
                    let mut disconnecting_players = Vec::with_capacity(conn.aliases.len() + 1);
                    disconnecting_players.push(player_id);
                    for fake_id in conn.aliases.iter() {
                        if let Some(user_desc) = self.user_descriptor(*fake_id) {
                            user_desc.state.mode.store(ConnectionMode::Disconnected.to_u8(), std::sync::atomic::Ordering::Relaxed);
                            let conn_opt = self.users.write().await.remove(fake_id);
                            if let Some(conn) = conn_opt {
                                if self.custom_logic_handler.on_player_end(self, &conn, user_desc).await {
                                    disconnecting_players.push(*fake_id);
                                }
                            }
                        }
                    }
                    let is_game_complete = self.is_complete.load(std::sync::atomic::Ordering::Relaxed);
                    if is_game_complete {
                        // save score to database as they disconnect
                        // this happens before they return to the main menu
                        let scores = user_info.counters.as_core();
                        match conn.user.update_game_score(self.game_guid(), scores).await {
                            Ok(score_id) => user_info.counters.score_id.store(score_id as i64, std::sync::atomic::Ordering::Relaxed),
                            Err(e) => {
                                log::warn!("Failed to save score for player {} (user {}) after end of game {}: {}", player_id, user_id, self.game_guid(), e);
                            },
                        }
                        // in every other case this packet would've already been sent
                        // this makes the end-of-match "continue" button send you back to the main menu a bit sooner
                        // (otherwise it waits for the multiplayer server to disconnect via timeout)
                        crate::events::log_lnl_send_failure(conn.connection.rlnl().send_empty(
                            rlnl::event_code::NetworkEvent::PlayerQuitRequestComplete,
                            literustlib::packet::Property::ReliableOrdered,
                            &conn.connection.connection,
                        ).await);
                    } else {
                        for disconnecter in disconnecting_players {
                            self.broadcast(
                                rlnl::event_code::NetworkEvent::OnAnotherClientDisconnected,
                                literustlib::packet::Property::ReliableOrdered,
                                &rlnl::events::ingame::PlayerId { player: disconnecter },
                                true,
                            ).await;
                        }
                    }

                    let mut has_active_connections = false;
                    for user in self.descriptors.values() {
                        if user.descriptor.user_id.is_none() { continue; } // skip non-players
                        let mode = ConnectionMode::from_u8(user.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                        has_active_connections |= !matches!(mode, ConnectionMode::Disconnected);
                    }
                    //is_engaged = has_active_connections;
                    if !has_active_connections {
                        self.is_complete.store(true, std::sync::atomic::Ordering::Relaxed);
                        if self.custom_logic_handler.on_game_completed(self).await {
                            if let Err(e) = conn.user.complete_game(self.game_guid()).await {
                                log::error!("Failed to mark game {} as complete: {}", self.game_guid(), e);
                            }
                        }
                    }
                    if let Err(e) = conn.user.save_player_connected_status(self.game_guid(), false).await {
                        log::error!("Failed to mark player {} (user {}) as disconnected in game {}: {}", player_id, user_id, self.game_guid(), e);
                    }
                    if !is_unregister || conn.connection.connection.is_connected() {
                        conn.connection.connection.goodbye(&conn.connection.sender).await;
                    }
                    return has_active_connections;
                }
            }
        }
        for user in self.descriptors.values() {
            if user.descriptor.user_id.is_none() { continue; } // skip non-players
            let mode = ConnectionMode::from_u8(user.state.mode.load(std::sync::atomic::Ordering::Relaxed));
            if !matches!(mode, ConnectionMode::Disconnected) {
                return true;
            }
        }
        false
    }

    async fn on_request_leave(&self, user_id: i32) {
        log::info!("User {} wants to leave game {}", user_id, self.game_guid());
        if let Some(player_id) = self.user_key_by_user_id(user_id) {
            self.rebroadcast(
                user_id,
                rlnl::event_code::NetworkEvent::MachineDestroyedConfirmed,
                literustlib::packet::Property::ReliableOrdered,
                &rlnl::events::ingame::Kill {
                    killee_player_id: player_id,
                    killer_player_id: player_id,
                },
                true,
            ).await;
            if let Some(conn) = self.users.read().await.get(&player_id) {
                crate::events::log_lnl_send_failure(conn.connection.rlnl().send_empty(
                    rlnl::event_code::NetworkEvent::PlayerQuitRequestComplete,
                    literustlib::packet::Property::ReliableOrdered,
                    &conn.connection.connection,
                ).await);
            }
        }
    }

    async fn on_loading_progress(&self, user_id: i32, user_name: String, progress: f32) {
        let progress_data = rlnl::events::loading::LoadingProgress {
            user_name: rlnl::types::BinaryWriterString(user_name),
            progress,
        };
        for (player_id, conn) in self.users.read().await.iter() {
            let user_desc = self.user_descriptor(*player_id).unwrap();
            if user_id == conn.user.account_id() {
                let progress_percent = ((progress * 100.0).ceil() as u8).clamp(0, 100);
                log::debug!("User {} is loaded {}% into game {}", user_id, progress_percent, self.game_guid());
                user_desc.state.progress.store(progress_percent, std::sync::atomic::Ordering::Relaxed);
                continue;
            }
            let mode = ConnectionMode::from_u8(user_desc.state.mode.load(std::sync::atomic::Ordering::Relaxed));
            match mode {
                ConnectionMode::Loading | ConnectionMode::Disconnected => {},
                ConnectionMode::WaitingForSync | ConnectionMode::Sync | ConnectionMode::WaitingToStart => {
                    crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                        &progress_data,
                        rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                        literustlib::packet::Property::ReliableOrdered,
                        &conn.connection.connection,
                    ).await);
                },
                ConnectionMode::InGame => {
                    log::warn!("Got loading progress for user {} who is supposed to be already in-game", user_id);
                },
            }
        }
    }

    async fn on_request_loading_progress(&self, user_id: i32) {
        //log::info!("Got request loading progress");
        if let Some(user_key) = self.user_key_by_user_id(user_id) {
            if let Some(user_info) = self.users.read().await.get(&user_key) {
                let mut client_ai_map = self.fakes_handler.get_client_ais().await;
                self.spawn_send_loading_events(user_info, user_key, self.players_info(), client_ai_map.remove(&user_key).unwrap_or_default());
                let sender = user_info.connection.rlnl();
                for user_desc in self.descriptors.values() {
                    if Some(user_id) == user_desc.descriptor.user_id { continue; }
                    let event = rlnl::events::loading::LoadingProgress {
                        //user_name: rlnl::types::BinaryWriterString(conn.user.user_name().to_owned()),
                        user_name: rlnl::types::BinaryWriterString(user_desc.descriptor.public_id.clone()),
                        progress: (user_desc.state.progress.load(std::sync::atomic::Ordering::Relaxed) as f32) / 100.0,
                    };
                    crate::events::log_lnl_send_failure(sender.send_data(
                        &event,
                        rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                        literustlib::packet::Property::ReliableOrdered,
                        &user_info.connection.connection,
                    ).await);
                }
            } else {
                log::error!("Failed to find player {} in connected users for match {}", user_key, self.game_guid());
            }
        } else {
            log::error!("Failed to find user {} in connected users for match {}", user_id, self.game_guid());
        }
    }

    async fn on_weapon_select(&self,
        user_id: i32,
        machine_id: u8,
        category: oj_rc_core::data::weapon_list::ItemCategory,
        size: oj_rc_core::data::cube_list::ItemTier
    ) {
        if let Some(conn) = self.user_descriptor(machine_id) {
            let category_u32 = category as u32;
            let size_u32 = size as u32;
            conn.machine.selected_weapon.category.store(category_u32, std::sync::atomic::Ordering::Relaxed);
            conn.machine.selected_weapon.size.store(size_u32, std::sync::atomic::Ordering::Relaxed);
            let data = rlnl::events::ingame::SelectWeapon {
                machine_id,
                item_category: category_u32,
                item_size: size_u32,
            };
            self.rebroadcast(
                user_id,
                rlnl::event_code::NetworkEvent::BroadcastWeaponSelect,
                literustlib::packet::Property::ReliableOrdered,
                &data,
                true
            ).await;
        }
    }

    async fn start_loading_sync_timeouter(&self) {
        let game_state = self.loading_state.clone();
        let tx = self.self_sender.clone().unwrap();
        if let Some(loading_autostart_after) = self.mp_config.loading_autostart_after {
            super::modes::trackers::Timeout::new(loading_autostart_after)
                .on_timeout(move || {
                    // start sync
                    let tx_clone = tx.clone();
                    let (tx_oneshot, rx_oneshot) = tokio::sync::oneshot::channel();
                    tokio::spawn(async move {
                        tx_clone.send(crate::matches::GameMessage::LoadingTimeout {
                            timeout: crate::matches::messages::TimeoutVariant::WaitingForLoadingSync,
                            response: tx_oneshot,
                        }).await.unwrap_or_default();
                    });
                    rx_oneshot.blocking_recv().unwrap_or(true)
                })
                .with_cancel_check(move || {
                    // check if sync is already started
                    let state = LoadingState::from_u8(game_state.load(std::sync::atomic::Ordering::Relaxed));
                    !matches!(state, LoadingState::Starting)
                })
                .start().await;
        }
    }

    async fn start_game_start_timeouter(&self) {
        let game_state = self.loading_state.clone();
        let tx = self.self_sender.clone().unwrap();
        if let Some(loading_autostart_after) = self.mp_config.loading_autostart_after {
            super::modes::trackers::Timeout::new(loading_autostart_after)
                .on_timeout(move || {
                    // start sync
                    let tx_clone = tx.clone();
                    let (tx_oneshot, rx_oneshot) = tokio::sync::oneshot::channel();
                    tokio::spawn(async move {
                        tx_clone.send(crate::matches::GameMessage::LoadingTimeout {
                            timeout: crate::matches::messages::TimeoutVariant::WaitingForGameStart,
                            response: tx_oneshot,
                        }).await.unwrap_or_default();
                    });
                    rx_oneshot.blocking_recv().unwrap_or(true)
                })
                .with_cancel_check(move || {
                    // check if sync is already started
                    let state = LoadingState::from_u8(game_state.load(std::sync::atomic::Ordering::Relaxed));
                    !matches!(state, LoadingState::InSync)
                })
                .start().await;
        }
    }

    async fn on_request_loading_sync(&self, user_id: i32) {
        // wait for all users to be ready before transitioning to loading sync
        let mut ready_count = 0;
        for (player_id, user) in self.users.read().await.iter() {
            let user_desc = self.user_descriptor(*player_id).unwrap();
            if user.aliases.contains(player_id) { continue; }
            if user.user.account_id() == user_id  {
                if !matches!(ConnectionMode::from_u8(user_desc.state.mode.load(std::sync::atomic::Ordering::Relaxed)), ConnectionMode::Loading | ConnectionMode::Disconnected) {
                    log::warn!("Got RequestLoadingSync after user {} was already in/past WaitingForSync stage", user_id);
                    continue;
                }
                log::info!("User {} (player {}) is awaiting sync in game {}", user_id, player_id, self.game_guid());
                user_desc.state.mode.store(ConnectionMode::WaitingForSync.to_u8(), std::sync::atomic::Ordering::Relaxed);
                ready_count += 1;
            } else if matches!(ConnectionMode::from_u8(user_desc.state.mode.load(std::sync::atomic::Ordering::Relaxed)), ConnectionMode::WaitingForSync) {
                ready_count += 1;
            }
        }
        let player_count = self.real_player_count();
        //log::info!("Real players {}, ready players {}", player_count, ready_count);
        if ready_count == player_count {
            self.start_sync().await;
        }
    }

    async fn start_sync(&self) {
        self.loading_state.store(LoadingState::InSync.to_u8(), std::sync::atomic::Ordering::Relaxed);
        let ready_players = count_users_in_mode(ConnectionMode::WaitingForSync, self.descriptors.values());
        let player_count = self.real_player_count();
        log::info!("Starting sync for {}/{} players in game {}", ready_players, player_count, self.game_guid());
        let mut to_disconnect = Vec::with_capacity(player_count - ready_players);
        for (user_key, conn) in self.users.read().await.iter() {
            let user_info = self.user_descriptor(*user_key).unwrap();
            let extra_packets = self.custom_logic_handler.extra_sync_events(self, conn, user_info).await;
            let user_desc = self.user_descriptor(*user_key).unwrap();
            self.spawn_send_sync_events(conn, user_desc.descriptor.user_id, *user_key, self.players_info(), extra_packets, self.map_config.clone());
            let old_mode = user_desc.state.mode.swap(ConnectionMode::Sync.to_u8(), std::sync::atomic::Ordering::Relaxed);
            let old_mode = ConnectionMode::from_u8(old_mode);
            if !matches!(old_mode, ConnectionMode::WaitingForSync) {
                if let Some(user_id) = user_desc.descriptor.user_id {
                    to_disconnect.push(user_id);
                    //conn.connection.connection.goodbye(&conn.connection.sender).await;
                }
            }
        }
        for user_id in to_disconnect {
            self.on_end_connection(user_id, false).await;
        }
        self.start_game_start_timeouter().await;
    }

    async fn on_load_complete(&self, user_id: i32) {
        if let Some(user_key) = self.user_key_by_user_id(user_id) {
            if let Some(conn) = self.users.read().await.get(&user_key) {
                let user_desc = self.user_descriptor(user_key).unwrap();
                log::info!("Loading complete for game {}, user {} (player {})", self.game_guid(), user_id, user_key);
                user_desc.state.progress.store(100, std::sync::atomic::Ordering::Relaxed);
                let mode = ConnectionMode::from_u8(user_desc.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                if !matches!(mode, ConnectionMode::Sync) {
                    log::warn!("Player {} completed loading but is in mode {:?} (should be Sync)", user_desc.descriptor.player_id, mode);
                }
                user_desc.state.mode.store(ConnectionMode::WaitingToStart.to_u8(), std::sync::atomic::Ordering::Relaxed);
                self.spawn_initial_ingame_events(conn, user_id);
            } else {
                log::warn!("Invalid LoadComplete user key {} for game {}", user_key, self.game_guid());
                return;
            }
        } else {
            log::warn!("Unknown LoadComplete user id {} for game {}", user_id, self.game_guid());
            return;
        }
        // wait for all users to be ready for starting game start countdown
        let all_users_loading_complete = is_all_users_in_mode(ConnectionMode::WaitingToStart, self.descriptors.values());
        // trigger game start
        if all_users_loading_complete {
            self.start_game().await;
        }
    }

    async fn start_game(&self) {
        self.loading_state.store(LoadingState::InGame.to_u8(), std::sync::atomic::Ordering::Relaxed);
        let ready_players = count_users_in_mode(ConnectionMode::WaitingToStart, self.descriptors.values());
        let player_count = self.real_player_count();
        log::info!("Starting game for {}/{} players in game {}", ready_players, player_count, self.game_guid());
        tokio::time::sleep(Self::END_OF_SYNC_DELAY).await;
        self.fakes_handler.on_ready(
            self.users.read().await.iter()
                .map(|(id, real_player)| (*id, real_player.connection.clone()))
                .collect()
        );
        let game_start = chrono::Utc::now() + Self::COUNTDOWN_DURATION;
        if self.custom_logic_handler.on_countdown_start(self, game_start).await {
            let mut senders = Vec::with_capacity(self.descriptors.len());
            let mut to_disconnect = Vec::with_capacity(player_count - ready_players);
            for (player_id, conn) in self.users.read().await.iter() {
                let user_desc = self.user_descriptor(*player_id).unwrap();
                if let Some(user_id) = user_desc.descriptor.user_id {
                    let mode = ConnectionMode::from_u8(user_desc.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                    if !matches!(mode, ConnectionMode::WaitingToStart) {
                        to_disconnect.push(user_id);
                    }
                }
                senders.push((conn.connection.clone(), user_desc.state.clone()));
            }
            self.game_start.store(game_start.timestamp(), std::sync::atomic::Ordering::Relaxed);
            super::countdown::match_countdown(senders, game_start);
            for user_id in to_disconnect {
                self.on_end_connection(user_id, false).await;
            }
        }
    }

    async fn on_spot_vehicle(&self, user_id: i32, remote_player: u8) {
        if self.custom_logic_handler.on_spot_vehicle(self, user_id, remote_player).await {
            self.rebroadcast(
                user_id,
                rlnl::event_code::NetworkEvent::RemoteEnemySpotted,
                literustlib::packet::Property::ReliableOrdered,
                &rlnl::events::ingame::PlayerId { player: remote_player },
                true
            ).await;
        }
    }

    async fn on_destroy_vehicle(&self,
        user_id: i32,
        remote_player: u8,
        killer_player: u8,
    ) {
        // FIXME allow custom_logic_handler to override the MachineDestroyedConfirmed send
        self.rebroadcast(
            user_id,
            rlnl::event_code::NetworkEvent::MachineDestroyedConfirmed,
            literustlib::packet::Property::ReliableOrdered,
            &rlnl::events::ingame::Kill { killee_player_id: remote_player, killer_player_id: killer_player },
            true,
        ).await;
        log::info!("Player {} was destroyed by player {} (user {}) in game {}", remote_player, killer_player, user_id, self.game_guid());
        if self.custom_logic_handler.on_vehicle_destroyed(self, killer_player, remote_player).await {
            // the kill tracking is initiated separately by the client with kill bonus event
            if let Some(killed) = self.user_descriptor(remote_player) {
                let was_killed = killed.machine.is_alive.swap(false, std::sync::atomic::Ordering::Relaxed);
                if was_killed {
                    killed.counters.deaths.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    killed.machine.is_alive.store(false, std::sync::atomic::Ordering::Relaxed);
                    let data = killed.counters.get_generic_packet(remote_player, rlnl::types::IngameStatId::RobotDestroyed, None);
                    self.broadcast(
                        rlnl::event_code::NetworkEvent::UpdateGameStats,
                        literustlib::packet::Property::ReliableOrdered,
                        &data,
                        true,
                    ).await;
                    //self.unclaimed.debounce_kill(KillAttribution { killer: killer_player, victim: remote_player }).await;
                }
            }
        }
    }

    async fn on_self_destruct(&self, user_id: i32, is_classic: bool) {
        if let Some(player_id) = self.user_key_by_user_id(user_id) {
            self.rebroadcast(
                user_id,
                rlnl::event_code::NetworkEvent::MachineDestroyedConfirmed,
                literustlib::packet::Property::ReliableOrdered,
                &rlnl::events::ingame::Kill { killee_player_id: player_id, killer_player_id: player_id },
                true,
            ).await;
            log::info!("Player {} ({}) self-destructed in game {} (elimination? {})", player_id, user_id, self.game_guid(), is_classic);
            if self.custom_logic_handler.on_vehicle_self_destruct(self, player_id, is_classic).await {
                if is_classic {
                    self.rebroadcast(
                        user_id,
                        rlnl::event_code::NetworkEvent::OnAnotherClientDisconnected,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::PlayerId { player: player_id },
                        true,
                    ).await;
                    if let Some(conn) = self.users.read().await.get(&player_id) {
                        crate::events::log_lnl_send_failure(conn.connection.rlnl().send_empty(
                            rlnl::event_code::NetworkEvent::PlayerQuitRequestComplete,
                            literustlib::packet::Property::ReliableOrdered,
                            &conn.connection.connection
                        ).await);
                        let user_desc = self.user_descriptor(player_id).unwrap();
                        user_desc.state.mode.store(ConnectionMode::Disconnected.to_u8(), std::sync::atomic::Ordering::Relaxed);
                        conn.connection.connection.disconnect();
                    }
                }
            }
        }
    }

    async fn on_flipping_started(&self, user_id: i32) {
        if let Some(user_key) = self.user_key_by_user_id(user_id) {
            self.rebroadcast(
                user_id,
                rlnl::event_code::NetworkEvent::AlignmentRectifierStarted,
                literustlib::packet::Property::ReliableOrdered,
                &rlnl::events::ingame::PlayerId { player: user_key },
                true,
            ).await;
        }
    }

    async fn on_map_ping(&self, _user_id: i32, ping: rlnl::events::ingame::MapPing) {
        for (id, conn) in self.users.read().await.iter() {
            let user_desc = self.user_descriptor(*id).unwrap();
            if (*id as i32) != ping.sender && user_desc.descriptor.team == ping.team_id {
                crate::events::log_lnl_send_failure(conn.connection.rlnl().send_data(
                    &ping,
                    rlnl::event_code::NetworkEvent::MapPingEvent,
                    literustlib::packet::Property::ReliableOrdered,
                    &conn.connection.connection
                ).await);
            }
        }
    }

    async fn on_kill_bonus(&self,
        _user_id: i32,
        shootee: u8,
        shooter: u8,
    ) {
        if self.custom_logic_handler.on_kill_bonus(self, shooter, shootee).await {
            if self.unclaimed.debounce_kill(KillAttribution { killer: shooter, victim: shootee }).await {
                if let Some(to_reward) = self.users.read().await.get(&shooter) {
                    let to_reward_desc = self.user_descriptor(shooter).unwrap();
                    to_reward_desc.counters.kills.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    crate::events::log_lnl_send_failure(to_reward.connection.rlnl().send_data(
                        &rlnl::events::ingame::Kill {
                            killee_player_id: shootee,
                            killer_player_id: shooter,
                        },
                        rlnl::event_code::NetworkEvent::ConfirmedKill,
                        literustlib::packet::Property::ReliableOrdered,
                        &to_reward.connection.connection
                    ).await);
                    let data = to_reward_desc.counters.get_generic_packet(shooter, rlnl::types::IngameStatId::Kill, None);
                    self.broadcast(
                        rlnl::event_code::NetworkEvent::UpdateGameStats,
                        literustlib::packet::Property::ReliableOrdered,
                        &data,
                        true,
                    ).await;
                }
            }
        }
    }

    async fn on_assist_bonus(&self,
        _user_id: i32,
        shootee: u8,
        shooters: Vec<u8>,
    ) {
        let lock = self.users.read().await;
        for shooter in shooters {
            if let Some(to_reward) = lock.get(&shooter) {
                let to_reward_desc = self.user_descriptor(shooter).unwrap();
                to_reward_desc.counters.assists.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                crate::events::log_lnl_send_failure(to_reward.connection.rlnl().send_data(
                    &rlnl::events::ingame::Kill {
                        killee_player_id: shootee,
                        killer_player_id: shooter,
                    },
                    rlnl::event_code::NetworkEvent::ConfirmedAssist,
                    literustlib::packet::Property::ReliableOrdered,
                    &to_reward.connection.connection
                ).await);
                let data = to_reward_desc.counters.get_generic_packet(shooter, rlnl::types::IngameStatId::KillAssist, None);
                self.broadcast(
                    rlnl::event_code::NetworkEvent::UpdateGameStats,
                    literustlib::packet::Property::ReliableOrdered,
                    &data,
                    true,
                ).await;
            }
        }
    }

    async fn on_heal_assist_bonus(&self,
        _user_id: i32,
        healer: u8,
        _healee: u8,
    ) {
        if let Some(to_reward_desc) = self.user_descriptor(healer) {
            //let to_reward_desc = self.user_descriptor(healee).unwrap();
            to_reward_desc.counters.heal_assists.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let data = to_reward_desc.counters.get_generic_packet(healer, rlnl::types::IngameStatId::HealAssist, None);
            self.broadcast(
                rlnl::event_code::NetworkEvent::UpdateGameStats,
                literustlib::packet::Property::ReliableOrdered,
                &data,
                true,
            ).await;
        }
    }

    async fn on_destroy_cubes_bonus(&self,
        _user_id: i32,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus
    ) {
        for shooter in info.shooters {
            if let Some(to_reward) = self.user_descriptor(shooter.shooting_player_id) {
                let mut total_cubes = 0;
                for target in shooter.shooter_targets {
                    if let Some(to_punish) = self.user_descriptor(target.target_player_id) {
                        let mut total_cubes_received = 0;
                        for cubes in target.cube_amounts {
                            // TODO use cube_id for something!?
                            total_cubes += cubes.cube_count;
                            total_cubes_received += cubes.cube_count;
                        }
                        to_punish.counters.received_cubes.fetch_add(total_cubes_received, std::sync::atomic::Ordering::SeqCst);
                    }
                }
                to_reward.counters.cubes.fetch_add(total_cubes, std::sync::atomic::Ordering::SeqCst);
                let data = to_reward.counters.get_generic_packet(shooter.shooting_player_id, rlnl::types::IngameStatId::DestroyedCubes, Some(total_cubes));
                self.broadcast(
                    rlnl::event_code::NetworkEvent::UpdateGameStats,
                    literustlib::packet::Property::Unreliable,
                    &data,
                    true,
                ).await;
            }
        }
    }

    async fn on_heal_cubes_bonus(&self,
        _user_id: i32,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus,
    ) {
        for shooter in info.shooters {
            if let Some(to_reward) = self.user_descriptor(shooter.shooting_player_id) {
                let mut total_cubes = 0;
                for target in shooter.shooter_targets {
                    if let Some(to_punish) = self.user_descriptor(target.target_player_id) {
                        let mut total_cubes_received = 0;
                        for cubes in target.cube_amounts {
                            // TODO use cube_id for something!?
                            total_cubes += cubes.cube_count;
                            total_cubes_received += cubes.cube_count;
                        }
                        to_punish.counters.received_healed.fetch_add(total_cubes_received, std::sync::atomic::Ordering::SeqCst);
                    }
                }
                to_reward.counters.healed.fetch_add(total_cubes, std::sync::atomic::Ordering::SeqCst);
                let data = to_reward.counters.get_generic_packet(shooter.shooting_player_id, rlnl::types::IngameStatId::HealCubes, Some(total_cubes));
                self.broadcast(
                    rlnl::event_code::NetworkEvent::UpdateGameStats,
                    literustlib::packet::Property::Unreliable,
                    &data,
                    true,
                ).await;
            }
        }
    }

    async fn on_broadcast(&self,
        user_id: i32,
        event: rlnl::event_code::NetworkEvent,
        event_in: rlnl::event_code::NetworkEvent,
        property: literustlib::packet::Property,
        data: Option<Box<dyn crate::Broadcastable>>,
        skip_user: bool,
    ) {
        if self.custom_logic_handler.on_broadcast(self, user_id, event, event_in, property, &data, skip_user).await {
            #[allow(clippy::collapsible_else_if)]
            if let Some(data) = data {
                if skip_user {
                    self.rebroadcast(user_id, event, property, &*data, true).await;
                } else {
                    self.broadcast(event, property, &*data, true).await;
                }
            } else {
                if skip_user {
                    self.rebroadcast_dataless(user_id, event, property, true).await;
                } else {
                    self.broadcast_dataless(event, property, true).await;
                }
            }
        }
    }

    async fn on_player_input_changed(&self, user_id: i32, input: rlnl::events::ingame::PlayerIdAndInputData) {
        let full_payload = rlnl::events::ingame::MultiPlayerInputChanged {
            num_players: 1,
            changes: vec![input],
        };
        self.rebroadcast(
            user_id,
            rlnl::event_code::NetworkEvent::OnServerReceivedInputChange,
            literustlib::packet::Property::ReliableOrdered,
            &full_payload,
            true,
        ).await;
    }

    async fn on_motion(&self, user_id: i32, motion: rlnl::machine_motion::MachineMotion) {
        //let (looking_at_x, looking_at_y, looking_at_z) = motion.target_point.clone().into();
        //log::info!("Player {} looking at ({}, {}, {})", motion.player_id, looking_at_x, looking_at_y, looking_at_z);
        let (x, y, z) = motion.rb_state.rb_pos_rot.pos.into();
        let (x2, y2, z2) = motion.rb_state.center_of_mass.into();
        let (w3, x3, y3, z3) = motion.rb_state.rb_pos_rot.rot.into();
        let quat = num_quaternion::Quaternion::new(w3, x3, y3, z3);
        if let Some(unit_quat) = quat.normalize() {
            let coords = unit_quat.rotate_vector([x2, y2, z2]);
            let (x4, y4, z4) = (x + coords[0], y + coords[1], z + coords[2]);
            //log::debug!("Player {} world CoM is at (x, y, z) ({}, {}, {})", motion.player_id, x4, y4, z4);
            if self.custom_logic_handler.on_motion(self, &motion, (x4, y4, z4)).await {
                if let Some(user_desc) = self.user_descriptor(motion.player_id) {
                    user_desc.machine.location.x.store(x4, std::sync::atomic::Ordering::Relaxed);
                    user_desc.machine.location.y.store(y4, std::sync::atomic::Ordering::Relaxed);
                    user_desc.machine.location.z.store(z4, std::sync::atomic::Ordering::Relaxed);
                    use byteserde::ser_heap::ByteSerializeHeap;
                    let mut ser = byteserde::ser_heap::ByteSerializerHeap::default();
                    if let Err(e) = motion.byte_serialize_heap(&mut ser) {
                        log::error!("Failed to serialize motion data from user {}: {}", user_id, e);
                    } else {
                        let data = bytes::Bytes::copy_from_slice(ser.as_slice());
                        for (player_id, conn) in self.users.read().await.iter() {
                            if *player_id == motion.player_id || conn.aliases.contains(player_id) || conn.aliases.contains(&motion.player_id) { continue; } // don't re-send to client AIs
                            // fun fact: the game sometimes hard crashes if you send its own motion data back to it
                            crate::events::log_lnl_send_failure(conn.connection.sender.send_data(crate::handler::EventData {
                                message_ty: crate::data::MessageType::RobotMotion,
                                variant: 0,
                                data_size: data.len() as _,
                                data: data.clone(),
                            }, literustlib::packet::Property::Unreliable, &conn.connection.connection).await);
                        }
                    }
                } else {
                    log::warn!("Received machine motion with unknown player id {} from user {}", motion.player_id, user_id);
                }
            }
        }
    }

    async fn on_timeout(&self, timeout: super::messages::TimeoutVariant, response: tokio::sync::oneshot::Sender<bool>) {
        let loading_state = LoadingState::from_u8(self.loading_state.load(std::sync::atomic::Ordering::Relaxed));
        match timeout {
            super::messages::TimeoutVariant::WaitingForLoadingSync => {
                if matches!(loading_state, LoadingState::Starting) {
                    let is_any_users_waiting = is_any_users_in_mode(ConnectionMode::WaitingForSync, self.descriptors.values());
                    if is_any_users_waiting {
                        response.send(true).unwrap_or_default();
                        log::info!("Reached max time waiting for loading sync, starting sync for game {}", self.game_guid());
                        self.start_sync().await;
                    } else {
                        response.send(false).unwrap_or_default();
                    }
                } else {
                    response.send(true).unwrap_or_default();
                }
            },
            super::messages::TimeoutVariant::WaitingForGameStart => {
                if matches!(loading_state, LoadingState::InSync) {
                    let is_any_users_waiting = is_any_users_in_mode(ConnectionMode::WaitingToStart, self.descriptors.values());
                    if is_any_users_waiting {
                        response.send(true).unwrap_or_default();
                        log::info!("Reached max time waiting for game start, starting countdown for game {}", self.game_guid());
                        self.start_game().await;
                    } else {
                        response.send(false).unwrap_or_default();
                    }
                } else {
                    response.send(true).unwrap_or_default();
                }
            }
        }
    }

    fn spawn_send_loading_events(&self, user: &UserConnection, player_id: u8, players: Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>>, client_ais: Vec<u8>) {
        let connection = user.connection.clone();
        let user_id = user.user.account_id();
        tokio::spawn(Self::send_loading_events_wrapper(connection, player_id, user_id, players, client_ais));
    }

    async fn send_loading_events_wrapper(connection: UserSender, player_id: u8, user_id: i32, players: Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>>, client_ais: Vec<u8>) {
        if let Err(e) = Self::send_loading_events(&connection, player_id, players, client_ais).await {
            log::error!("Failed to send Loading events for user {} ({}): {}", user_id, player_id, e);
        }
    }

    async fn send_loading_events(user: &UserSender, _player_id: u8, players: Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>>, client_ais: Vec<u8>) -> std::io::Result<()> {
        //tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        let sender = user.rlnl();
        /*sender.send_data(
            &rlnl::events::ingame::PlayerId { player: player_id },
            rlnl::event_code::NetworkEvent::GameGuidValidated,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection
        ).await?;*/
        sender.send_data(
            &rlnl::events::loading::PlayerIDsAndNames {
                num_players: players.len() as _,
                players: players.iter().map(|player| rlnl::events::loading::PlayerIDAndName {
                    player_id: player.player_id as _,
                    name: rlnl::types::BinaryWriterString(player.public_id.clone()),
                    display_name: rlnl::types::BinaryWriterString(player.display_name.clone()),
                })
                .collect(),
            },
            rlnl::event_code::NetworkEvent::PlayerIDs,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection
        ).await?;
        sender.send_data(
            &rlnl::events::loading::PlayerIDs {
                num_ids: client_ais.len() as i32,
                players: client_ais.into_iter().map(|x| x as i32).collect(),
            },
            rlnl::event_code::NetworkEvent::HostAIs,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection
        ).await?;
        Ok(())
    }

    fn spawn_send_sync_events(&self, user: &UserConnection, user_id: Option<i32>, player_id: u8, players: Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>>, extra_packets: Vec<super::RlnlPacket>, map: std::sync::Arc<oj_rc_core::persist::config::MapConfig>) {
        let connection = user.connection.clone();
        tokio::spawn(Self::send_sync_events_wrapper(connection, user_id, player_id, players, extra_packets, map));
        //user.state.mode.store(ConnectionMode::Sync.to_u8(), std::sync::atomic::Ordering::Relaxed);
    }

    async fn send_sync_events_wrapper(connection: UserSender, user_id: Option<i32>, player_id: u8, players: Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>>, extra_packets: Vec<super::RlnlPacket>, map: std::sync::Arc<oj_rc_core::persist::config::MapConfig>) {
        if let Err(e) = Self::send_sync_events(connection, player_id, players, extra_packets, map).await {
            log::error!("Failed to send Sync events for user {}: {}", user_id.unwrap_or(-1), e);
        }
    }

    async fn send_sync_events(connection: UserSender, _player_id: u8, players: Vec<std::sync::Arc<oj_rc_core::persist::user::PlayerDescriptor>>, extra_packets: Vec<super::RlnlPacket>, map: std::sync::Arc<oj_rc_core::persist::config::MapConfig>) -> std::io::Result<()> {
        let num_players = players.len() as u8;
        let sender = connection.rlnl();
        sender.send_empty(
            rlnl::event_code::NetworkEvent::BeginSync,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;

        let spawn_data_already_handled = extra_packets.iter().any(|packet| matches!(packet.event, rlnl::event_code::NetworkEvent::FreeSpawnPoint));

        for packet in extra_packets {
            sender.send_data(
                &*packet.data,
                packet.event,
                packet.property,
                &connection.connection)
            .await?;
        }

        sender.send_data(
            &rlnl::events::sync::InitialiseGameStats {
                num_players,
                stats: (0..num_players)
                    .map(|i| rlnl::types::IngamePlayerStats {
                        player_name: i,
                        num_stats: 0,
                        stats: vec![],
                    }).collect(),
            },
            rlnl::event_code::NetworkEvent::InitialiseGameStats,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;
        if !spawn_data_already_handled {
            if map.spawns.is_empty() {
                // fallback
                for i in 0..num_players {
                    sender.send_data(
                        &rlnl::events::sync::SpawnPoint {
                            pos: rlnl::types::PosQuatPair {
                                pos: rlnl::types::CompressedVec3::from((10.0 * (i as f32), 100.0, 10.0 * (i as f32))),
                                rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                            },
                            owner: i,
                        },
                        rlnl::event_code::NetworkEvent::FreeSpawnPoint,
                        literustlib::packet::Property::ReliableOrdered,
                        &connection.connection)
                    .await?;
                }
            } else {
                let mut last_spawn_point = std::collections::HashMap::with_capacity(2); // team -> last index
                for player in players.iter() {
                    let team = player.team as u8;
                    if let Some(team_points) = map.spawns.get(&team) {
                        if !team_points.is_empty() {
                            let spawn_index = if let Some(last_spawn_i) = last_spawn_point.get_mut(&team) {
                                *last_spawn_i = (*last_spawn_i + 1) % team_points.len();
                                *last_spawn_i
                            } else {
                                last_spawn_point.insert(team, 0usize);
                                0
                            };
                            let spawn = &team_points[spawn_index];
                            sender.send_data(
                                &rlnl::events::sync::SpawnPoint {
                                    pos: rlnl::types::PosQuatPair {
                                        pos: rlnl::types::CompressedVec3::from((spawn.x, spawn.y, spawn.z)),
                                        rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                                    },
                                    owner: player.player_id,
                                },
                                rlnl::event_code::NetworkEvent::FreeSpawnPoint,
                                literustlib::packet::Property::ReliableOrdered,
                                &connection.connection)
                            .await?;
                            continue;
                        }
                    }
                    // fallback
                    log::warn!("No spawn point found for player {} on team {}, using bad fallback", player.player_id, team);
                    sender.send_data(
                        &rlnl::events::sync::SpawnPoint {
                            pos: rlnl::types::PosQuatPair {
                                pos: rlnl::types::CompressedVec3::from((10.0 * (player.player_id as f32), 100.0, 10.0 * (team as f32) + 10.0)),
                                rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                            },
                            owner: player.player_id,
                        },
                        rlnl::event_code::NetworkEvent::FreeSpawnPoint,
                        literustlib::packet::Property::ReliableOrdered,
                        &connection.connection)
                    .await?;

                }
            }
        }


        // seems to be for reconnecting
        /*sender.send_data(
            &rlnl::events::sync::SyncMachineCubes {
                machine_id: 0,
                num_cubes: 0,
                events: vec![
                    rlnl::types::CubeState {
                        loc: rlnl::types::Byte3 { x: 0, y: 0, z: 0 },
                        status: rlnl::types::CubeStatus {
                            ty: rlnl::types::CubeHistoryEventType::Heal,
                            damage: Some(1),
                        }
                    }
                ],
            },
            rlnl::event_code::NetworkEvent::SyncMachineCubes,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection)
        .await?;*/

        tokio::time::sleep(GenericGamemodeEngine::<super::modes::NoOpLogic>::END_OF_SYNC_DELAY).await;

        sender.send_empty(
            rlnl::event_code::NetworkEvent::EndOfSync,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection
        ).await?;
        Ok(())
    }

    fn spawn_initial_ingame_events(&self, user: &UserConnection, user_id: i32) {
        let connection = user.connection.clone();
        tokio::spawn(Self::send_initial_ingame_events_wrapper(connection, user_id));
        //user.state.mode.store(ConnectionMode::InGame.to_u8(), std::sync::atomic::Ordering::Relaxed);
    }

    async fn send_initial_ingame_events_wrapper(connection: UserSender, user_id: i32) {
        if let Err(e) = Self::send_initial_ingame_events(connection).await {
            log::error!("Failed to send Sync events for user {}: {}", user_id, e);
        }
    }

    async fn send_initial_ingame_events(_connection: UserSender) -> std::io::Result<()> {
        //let sender = connection.rlnl();
        /*sender.send_data(
            &rlnl::events::GameTime(3.0),
            rlnl::event_code::NetworkEvent::TimeToGameStart,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;*/
        Ok(())
    }

    pub(super) fn game_done(&self) {
        let old = self.is_complete.swap(true, std::sync::atomic::Ordering::SeqCst);
        if old {
            log::warn!("Game {} was marked as done again", self.game_guid());
        } else {
            //panic!("Game should not be done!");
            log::debug!("Game {} is marked done (handler will exit once all players have disconnected)", self.game_guid());
        }
    }

    pub(super) fn is_game_done(&self) -> bool {
        self.is_complete.load(std::sync::atomic::Ordering::SeqCst)
    }

    #[inline]
    pub(super) fn is_in(loc: &(f32, f32, f32), sphere: &oj_rc_core::persist::config::Sphere) -> bool {
        let distance = (
            (loc.0 - sphere.center.x).powi(2)
            + (loc.1 - sphere.center.y).powi(2)
            + (loc.2 - sphere.center.z).powi(2)
        ).sqrt();
        //log::info!("{} away from sphere", distance);
        distance < sphere.radius
    }
}

fn count_users_in_mode<'a>(wants_mode: ConnectionMode, descriptors: impl std::iter::Iterator<Item = &'a UserDescriptor>) -> usize {
    descriptors.filter(|desc|
        desc.descriptor.user_id.is_some()
            && desc.state.mode.load(std::sync::atomic::Ordering::Relaxed) == wants_mode.to_u8()
    ).count()
}

fn is_any_users_in_mode<'a>(wants_mode: ConnectionMode, mut descriptors: impl std::iter::Iterator<Item = &'a UserDescriptor>) -> bool {
    descriptors.any(|desc|
        desc.descriptor.user_id.is_some()
            && desc.state.mode.load(std::sync::atomic::Ordering::Relaxed) == wants_mode.to_u8()
    )
}

fn is_all_users_in_mode<'a>(wants_mode: ConnectionMode, mut descriptors: impl std::iter::Iterator<Item = &'a UserDescriptor>) -> bool {
    descriptors.all(|desc| {
        let mode = ConnectionMode::from_u8(desc.state.mode.load(std::sync::atomic::Ordering::Relaxed));
        desc.descriptor.user_id.is_none()
        || (
            desc.descriptor.user_id.is_some()
            && (mode.to_u8() == wants_mode.to_u8() || matches!(mode, ConnectionMode::Disconnected))
        )
    })
}
