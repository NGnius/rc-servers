pub(super) struct UserConnection {
    pub(super) user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
    pub(super) connection: UserSender,
    pub(super) state: std::sync::Arc<UserState>,
    pub(super) machine: MachineState,
    pub(super) descriptor: oj_rc_core::persist::user::PlayerDescriptor,
    pub(super) counters: UserData,
}

#[allow(dead_code)]
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
}

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
}

impl MachineState {
    fn new() -> Self {
        Self {
            selected_weapon: WeaponInfo::new(),
            location: Location::new(),
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
    pub kills: std::sync::atomic::AtomicU32,
    pub deaths: std::sync::atomic::AtomicU32,
    pub assists: std::sync::atomic::AtomicU32,
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
            kills: std::sync::atomic::AtomicU32::new(0),
            deaths: std::sync::atomic::AtomicU32::new(0),
            assists: std::sync::atomic::AtomicU32::new(0),
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
            rlnl::types::IngameStatId::HealCubes => (self.assists.load(std::sync::atomic::Ordering::SeqCst), 1),
            rlnl::types::IngameStatId::RobotDestroyed => (self.deaths.load(std::sync::atomic::Ordering::Relaxed), 0),
            rlnl::types::IngameStatId::DestroyedProtoniumCubes => (self.deaths.load(std::sync::atomic::Ordering::Relaxed), 25),
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

pub(super) struct GenericGamemodeEngine<L: super::CustomGameLogic> {
    pub users: tokio::sync::RwLock<std::collections::HashMap<u8, UserConnection>>,
    pub user_id_map: tokio::sync::RwLock<std::collections::HashMap<i32, u8>>,
    //pub recv: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<super::GameMessage>>,
    //pub send: tokio::sync::mpsc::Sender<super::GameMessage>,
    //pub game_guid: String,
    is_complete: std::sync::atomic::AtomicBool,
    pub game_start: std::sync::atomic::AtomicI64,
    pub map_config: std::sync::Arc<oj_rc_core::persist::config::MapConfig>,
    pub game_descriptor: oj_rc_core::persist::user::GameDescriptor,
    pub game_duration: std::time::Duration,
    pub players_info: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>,
    pub custom_logic_handler: L,
    pub fake_users: std::collections::HashMap<u8, FakeUser>,
    pub fakes_handler: super::fake::Handler,
}

impl <L: super::CustomGameLogic> GenericGamemodeEngine<L> {
    const END_OF_SYNC_DELAY: std::time::Duration = std::time::Duration::from_millis(100);
    const COUNTDOWN_DURATION: std::time::Duration = std::time::Duration::from_secs(5);

    pub fn new(
        game: oj_rc_core::persist::user::GameDescriptor,
        map: oj_rc_core::persist::config::MapConfig,
        config: &oj_rc_core::data::game_mode::GameModeConfig,
        players: Vec<oj_rc_core::persist::user::PlayerDescriptor>,
        custom: L,
        fakes_handler: super::fake::Handler,
    ) -> Self {

        let fake_users = players.iter()
            .filter(|player| player.user_id.is_none())
            .map(|player| (player.team as u8, FakeUser::new(player.to_owned())))
            .collect();
        Self {
            users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            user_id_map: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            is_complete: std::sync::atomic::AtomicBool::new(false),
            game_start: std::sync::atomic::AtomicI64::new(-1),
            map_config: std::sync::Arc::new(map),
            game_descriptor: game,
            game_duration: std::time::Duration::from_secs((config.game_time_minutes as u64) * 60),
            players_info: std::sync::Arc::new(players),
            custom_logic_handler: custom,
            fake_users,
            fakes_handler,
        }
    }

    #[inline]
    pub(super) fn game_guid(&self) -> &'_ str {
        &self.game_descriptor.guid
    }

    pub(super) async fn user_key_by_user_id(&self, user_id: i32) -> Option<u8> {
        self.user_id_map.read().await.get(&user_id).copied()
    }

    pub(super) async fn rebroadcast<T: byteserde::ser_heap::ByteSerializeHeap + ?Sized>(&self, user_id: i32, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &T, in_game: bool) {
        for conn in self.users.read().await.values() {
            if user_id == conn.user.user_id() { continue; }
            if in_game {
                let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
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
        for conn in self.users.read().await.values() {
            if user_id == conn.user.user_id() { continue; }
            if in_game {
                let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
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
        for conn in self.users.read().await.values() {
            if in_game {
                let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
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
        for conn in self.users.read().await.values() {
            if in_game {
                let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
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

    /*pub(super) async fn send_to_player<T: byteserde::ser_heap::ByteSerializeHeap + ?Sized>(&self, player_id: u8, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: &T) {
        if let Some(player) = self.users.read().await.get(&player_id) {
            crate::events::log_lnl_send_failure(player.connection.rlnl().send_data(
                data,
                code,
                property,
                &player.connection.connection,
            ).await);
        }
    }*/

    pub(super) fn spawn(self) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(super::CHANNEL_BOUND);
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
                    super::GameMessage::EndConnection { user_id } => {
                        is_engaged = self.on_end_connection(user_id).await;
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
                        self.rebroadcast(
                            user_id,
                            rlnl::event_code::NetworkEvent::RemoteEnemySpotted,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::PlayerId { player: remote_player },
                            true
                        ).await;
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
                    super::GameMessage::Motion { user_id, motion } => {
                        self.on_motion(user_id, motion).await;
                    },
                    super::GameMessage::NoOp => {},
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
            let mut users = self.users.write().await;
            //tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            //let id = users.len() as u8;
            let user_id = user.user_id();
            let player_info = self.players_info.iter().find(|p| p.user_id == Some(user_id)).unwrap();
            let id = player_info.player_id;
            let new_user = UserConnection {
                user,
                connection: UserSender {
                    connection,
                    sender,
                },
                state: std::sync::Arc::new(UserState::new()),
                machine: MachineState::new(),
                descriptor: player_info.to_owned(),
                counters: UserData::new(),
            };
            if self.custom_logic_handler.on_player_join(self, &new_user, &self.players_info).await {
                //self.spawn_send_loading_events(&new_user, id, self.players_info.clone());
                crate::events::log_lnl_send_failure(new_user.connection.rlnl().send_data(
                    &rlnl::events::ingame::PlayerId { player: id },
                    rlnl::event_code::NetworkEvent::GameGuidValidated,
                    literustlib::packet::Property::ReliableOrdered,
                    &new_user.connection.connection
                ).await);
                log::debug!("User {} is validated to play game {}", new_user.user.user_id(), game_guid);
                self.user_id_map.write().await.insert(new_user.user.user_id(), id);
                users.insert(id, new_user);
            }
            response.send(None).unwrap_or_default();
        }
    }

    async fn on_end_connection(&self, user_id: i32) -> bool {
        if let Some(player_id) = self.user_key_by_user_id(user_id).await {
            let conn_opt = self.users.write().await.remove(&player_id);
            if let Some(conn) = conn_opt {
                if self.custom_logic_handler.on_player_end(self, &conn).await {
                    conn.state.mode.store(ConnectionMode::Disconnected.to_u8(), std::sync::atomic::Ordering::Relaxed);
                    if !self.is_complete.load(std::sync::atomic::Ordering::Relaxed) {
                        self.rebroadcast(
                            user_id,
                            rlnl::event_code::NetworkEvent::OnAnotherClientDisconnected,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::PlayerId { player: player_id },
                            true,
                        ).await;
                    } else {
                        // in every other case this packet would've already been sent
                        // this makes the end-of-match "continue" button send you back to the main menu a bit sooner
                        // (otherwise it waits for the multiplayer server to disconnect via timeout)
                        crate::events::log_lnl_send_failure(conn.connection.rlnl().send_empty(
                            rlnl::event_code::NetworkEvent::PlayerQuitRequestComplete,
                            literustlib::packet::Property::ReliableOrdered,
                            &conn.connection.connection,
                        ).await);
                    }
                    let mut has_active_connections = false;
                    for user in self.users.read().await.values() {
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
                    conn.connection.connection.goodbye(&conn.connection.sender).await;
                    return has_active_connections;
                }
            }
        }
        true
    }

    async fn on_request_leave(&self, user_id: i32) {
        log::info!("User {} wants to leave game {}", user_id, self.game_guid());
        if let Some(player_id) = self.user_key_by_user_id(user_id).await {
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
        for conn in self.users.read().await.values() {
            if user_id == conn.user.user_id() {
                let progress_percent = ((progress * 100.0).ceil() as u8).clamp(0, 100);
                log::info!("User {} is loaded {}% into game {}", user_id, progress_percent, self.game_guid());
                conn.state.progress.store(progress_percent, std::sync::atomic::Ordering::Relaxed);
                continue;
            }
            let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
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
        log::info!("Got request loading progress");
        if let Some(user_key) = self.user_key_by_user_id(user_id).await {
            if let Some(user_info) = self.users.read().await.get(&user_key) {
                self.spawn_send_loading_events(user_info, user_key, self.players_info.clone());
                let sender = user_info.connection.rlnl();
                for conn in self.users.read().await.values() {
                    if user_id == conn.user.user_id() { continue; }
                    /*crate::events::log_lnl_send_failure(sender.send_data(
                        &user_info.1,
                        rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                        literustlib::packet::Property::ReliableOrdered,
                        &user_info.0.connection,
                    ).await);*/
                    let event = rlnl::events::loading::LoadingProgress {
                        user_name: rlnl::types::BinaryWriterString(conn.user.user_name().to_owned()),
                        progress: (conn.state.progress.load(std::sync::atomic::Ordering::Relaxed) as f32) / 100.0,
                    };
                    crate::events::log_lnl_send_failure(sender.send_data(
                        &event,
                        rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                        literustlib::packet::Property::ReliableOrdered,
                        &user_info.connection.connection,
                    ).await)
                }
                for fake in self.fake_users.values() {
                    let event = rlnl::events::loading::LoadingProgress {
                        user_name: rlnl::types::BinaryWriterString(fake.descriptor.public_id.clone()),
                        progress: (fake.state.progress.load(std::sync::atomic::Ordering::Relaxed) as f32) / 100.0,
                    };
                    crate::events::log_lnl_send_failure(sender.send_data(
                        &event,
                        rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                        literustlib::packet::Property::ReliableOrdered,
                        &user_info.connection.connection,
                    ).await)
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
        if let Some(conn) = self.users.read().await.get(&machine_id) {
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

    async fn on_request_loading_sync(&self, user_id: i32) {
        // wait for all users to be ready before transitioning to loading sync
        let mut ready_count = 0;
        for user in self.users.read().await.values() {
            if user.user.user_id() == user_id {
                if !matches!(ConnectionMode::from_u8(user.state.mode.load(std::sync::atomic::Ordering::Relaxed)), ConnectionMode::Loading | ConnectionMode::Disconnected) {
                    log::warn!("Got RequestLoadingSync after user {} was already in/past WaitingForSync stage", user_id);
                    continue;
                }
                log::info!("User {} is awaiting sync", user_id);
                user.state.mode.store(ConnectionMode::WaitingForSync.to_u8(), std::sync::atomic::Ordering::Relaxed);
                ready_count += 1;
            } else if matches!(ConnectionMode::from_u8(user.state.mode.load(std::sync::atomic::Ordering::Relaxed)), ConnectionMode::WaitingForSync) {
                ready_count += 1;
            }
        }
        let player_count = self.players_info.iter().filter(|x| x.user_id.is_some()).count();
        if ready_count == player_count {
            log::info!("All players ({}) awaiting sync for game {}", player_count, self.game_guid());
            for (user_key, conn) in self.users.read().await.iter() {
                let extra_packets = self.custom_logic_handler.extra_sync_events(self, conn).await;
                self.spawn_send_sync_events(conn, conn.user.user_id(), *user_key, self.players_info.clone(), extra_packets, self.map_config.clone());
            }
        }
    }

    async fn on_load_complete(&self, user_id: i32) {
        if let Some(user_key) = self.user_key_by_user_id(user_id).await {
            if let Some(conn) = self.users.read().await.get(&user_key) {
                log::info!("Loading complete for game {}, user {} (player {})", self.game_guid(), user_id, user_key);
                conn.state.progress.store(100, std::sync::atomic::Ordering::Relaxed);
                let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                if !matches!(mode, ConnectionMode::Sync) {
                    log::warn!("Player {} completed loading but is in mode {:?} (should be Sync)", conn.descriptor.player_id, mode);
                }
                conn.state.mode.store(ConnectionMode::WaitingToStart.to_u8(), std::sync::atomic::Ordering::Relaxed);
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
        let mut all_users_loading_complete = true;
        for conn in self.users.read().await.values() {
            let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
            all_users_loading_complete &= matches!(mode, ConnectionMode::WaitingToStart);
        }
        // trigger game start
        if all_users_loading_complete {
            let player_count = self.players_info.iter().filter(|x| x.user_id.is_some()).count();
            log::info!("All players ({}) are ready for game {}", player_count, self.game_guid());
            tokio::time::sleep(Self::END_OF_SYNC_DELAY).await;
            self.fakes_handler.on_ready(
                self.users.read().await.iter()
                    .map(|(id, real_player)| (*id, real_player.connection.clone()))
                    .collect()
            );
            let game_start = chrono::Utc::now() + Self::COUNTDOWN_DURATION;
            if self.custom_logic_handler.on_countdown_start(self, game_start).await {
                let mut senders = Vec::new();
                for conn in self.users.read().await.values() {
                    senders.push((conn.connection.clone(), conn.state.clone()));
                }
                self.game_start.store(game_start.timestamp(), std::sync::atomic::Ordering::Relaxed);
                super::countdown::match_countdown(senders, game_start);
            }
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
            if let Some(killed) = self.users.read().await.get(&remote_player) {
                killed.counters.deaths.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let data = killed.counters.get_generic_packet(remote_player, rlnl::types::IngameStatId::RobotDestroyed, None);
                self.broadcast(
                    rlnl::event_code::NetworkEvent::UpdateGameStats,
                    literustlib::packet::Property::ReliableOrdered,
                    &data,
                    true,
                ).await;
            }
        }
    }

    async fn on_self_destruct(&self, user_id: i32, is_classic: bool) {
        if let Some(player_id) = self.user_key_by_user_id(user_id).await {
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
                        conn.state.mode.store(ConnectionMode::Disconnected.to_u8(), std::sync::atomic::Ordering::Relaxed);
                        conn.connection.connection.disconnect();
                    }
                }
            }
        }
    }

    async fn on_flipping_started(&self, user_id: i32) {
        if let Some(user_key) = self.user_key_by_user_id(user_id).await {
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
            if (*id as i32) != ping.sender && conn.descriptor.team == ping.team_id {
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
        if let Some(to_reward) = self.users.read().await.get(&shooter) {
            to_reward.counters.kills.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            crate::events::log_lnl_send_failure(to_reward.connection.rlnl().send_data(
                &rlnl::events::ingame::Kill {
                    killee_player_id: shootee,
                    killer_player_id: shooter,
                },
                rlnl::event_code::NetworkEvent::ConfirmedKill,
                literustlib::packet::Property::ReliableOrdered,
                &to_reward.connection.connection
            ).await);
            let data = to_reward.counters.get_generic_packet(shooter, rlnl::types::IngameStatId::Kill, None);
            self.broadcast(
                rlnl::event_code::NetworkEvent::UpdateGameStats,
                literustlib::packet::Property::ReliableOrdered,
                &data,
                true,
            ).await;
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
                to_reward.counters.assists.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                crate::events::log_lnl_send_failure(to_reward.connection.rlnl().send_data(
                    &rlnl::events::ingame::Kill {
                        killee_player_id: shootee,
                        killer_player_id: shooter,
                    },
                    rlnl::event_code::NetworkEvent::ConfirmedAssist,
                    literustlib::packet::Property::ReliableOrdered,
                    &to_reward.connection.connection
                ).await);
                let data = to_reward.counters.get_generic_packet(shooter, rlnl::types::IngameStatId::KillAssist, None);
                self.broadcast(
                    rlnl::event_code::NetworkEvent::UpdateGameStats,
                    literustlib::packet::Property::ReliableOrdered,
                    &data,
                    true,
                ).await;
            }
        }
    }

    async fn on_destroy_cubes_bonus(&self,
        _user_id: i32,
        info: rlnl::events::ingame::DestroyedHealedCubesBonus
    ) {
        let lock = self.users.read().await;
        for shooter in info.shooters {
            if let Some(to_reward) = lock.get(&shooter.shooting_player_id) {
                let mut total_cubes = 0;
                for target in shooter.shooter_targets {
                    if let Some(to_punish) = lock.get(&target.target_player_id) {
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
        let lock = self.users.read().await;
        for shooter in info.shooters {
            if let Some(to_reward) = lock.get(&shooter.shooting_player_id) {
                let mut total_cubes = 0;
                for target in shooter.shooter_targets {
                    if let Some(to_punish) = lock.get(&target.target_player_id) {
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
                if let Some(conn) = self.users.read().await.get(&motion.player_id) {
                    conn.machine.location.x.store(x4, std::sync::atomic::Ordering::Relaxed);
                    conn.machine.location.y.store(y4, std::sync::atomic::Ordering::Relaxed);
                    conn.machine.location.z.store(z4, std::sync::atomic::Ordering::Relaxed);
                    use byteserde::ser_heap::ByteSerializeHeap;
                    let mut ser = byteserde::ser_heap::ByteSerializerHeap::default();
                    if let Err(e) = motion.byte_serialize_heap(&mut ser) {
                        log::error!("Failed to serialize motion data from user {}: {}", user_id, e);
                    } else {
                        let data = bytes::Bytes::copy_from_slice(ser.as_slice());
                        for conn in self.users.read().await.values() {
                            if conn.user.user_id() == user_id { continue; } // fun fact: the game hard crashes if you omit this
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

    fn spawn_send_loading_events(&self, user: &UserConnection, player_id: u8, players: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>) {
        let connection = user.connection.clone();
        let user_id = user.user.user_id();
        tokio::spawn(Self::send_loading_events_wrapper(connection, player_id, user_id, players));
    }

    async fn send_loading_events_wrapper(connection: UserSender, player_id: u8, user_id: i32, players: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>) {
        if let Err(e) = Self::send_loading_events(&connection, player_id, players).await {
            log::error!("Failed to send Loading events for user {} ({}): {}", user_id, player_id, e);
        }
    }

    async fn send_loading_events(user: &UserSender, _player_id: u8, players: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>) -> std::io::Result<()> {
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
                num_ids: 0,
                players: vec![],
            },
            rlnl::event_code::NetworkEvent::HostAIs,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection
        ).await?;
        Ok(())
    }

    fn spawn_send_sync_events(&self, user: &UserConnection, user_id: i32, player_id: u8, players: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>, extra_packets: Vec<super::RlnlPacket>, map: std::sync::Arc<oj_rc_core::persist::config::MapConfig>) {
        let connection = user.connection.clone();
        tokio::spawn(Self::send_sync_events_wrapper(connection, user_id, player_id, players, extra_packets, map));
        user.state.mode.store(ConnectionMode::Sync.to_u8(), std::sync::atomic::Ordering::Relaxed);
    }

    async fn send_sync_events_wrapper(connection: UserSender, user_id: i32, player_id: u8, players: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>, extra_packets: Vec<super::RlnlPacket>, map: std::sync::Arc<oj_rc_core::persist::config::MapConfig>) {
        if let Err(e) = Self::send_sync_events(connection, player_id, players, extra_packets, map).await {
            log::error!("Failed to send Sync events for user {}: {}", user_id, e);
        }
    }

    async fn send_sync_events(connection: UserSender, _player_id: u8, players: std::sync::Arc<Vec<oj_rc_core::persist::user::PlayerDescriptor>>, extra_packets: Vec<super::RlnlPacket>, map: std::sync::Arc<oj_rc_core::persist::config::MapConfig>) -> std::io::Result<()> {
        let num_players = players.len() as u8;
        let sender = connection.rlnl();
        sender.send_empty(
            rlnl::event_code::NetworkEvent::BeginSync,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;

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

        // TODO
        Ok(())
    }

    pub(super) fn game_done(&self) {
        let old = self.is_complete.swap(true, std::sync::atomic::Ordering::SeqCst);
        if old {
            log::warn!("Game {} was marked as done again", self.game_guid());
        } else {
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
