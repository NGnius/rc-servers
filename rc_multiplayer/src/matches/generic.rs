pub(super) struct UserConnection {
    pub(super) user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
    pub(super) connection: UserSender,
    pub(super) state: std::sync::Arc<UserState>,
    pub(super) machine: MachineState,
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
}

impl MachineState {
    fn new() -> Self {
        Self {
            selected_weapon: WeaponInfo::new(),
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

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(super) enum ConnectionMode {
    Loading = 0,
    WaitingForSync = 1,
    Sync = 2,
    WaitingToStart = 3,
    InGame = 4,
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
            x => panic!("Unrecognized ConnectionMode {}", x),
        }
    }

    #[inline]
    pub(super) fn to_u8(self) -> u8 {
        self as u8
    }
}

pub(super) struct GenericGamemodeEngine {
    pub users: tokio::sync::RwLock<std::collections::HashMap<u8, UserConnection>>,
    pub user_id_map: tokio::sync::RwLock<std::collections::HashMap<i32, u8>>,
    //pub recv: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<super::GameMessage>>,
    //pub send: tokio::sync::mpsc::Sender<super::GameMessage>,
    pub game_guid: String,
    pub is_complete: std::sync::atomic::AtomicBool,
    pub game_start: std::sync::atomic::AtomicI64,
    pub player_count: std::sync::atomic::AtomicU8,
}

impl GenericGamemodeEngine {
    const END_OF_SYNC_DELAY: std::time::Duration = std::time::Duration::from_millis(100);
    const COUNTDOWN_DURATION: std::time::Duration = std::time::Duration::from_secs(5);

    pub fn new(guid: String) -> Self {

        Self {
            users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            user_id_map: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            game_guid: guid,
            is_complete: std::sync::atomic::AtomicBool::new(false),
            game_start: std::sync::atomic::AtomicI64::new(-1),
            player_count: std::sync::atomic::AtomicU8::new(0),
        }
    }

    pub(super) async fn user_key_by_user_id(&self, user_id: i32) -> Option<u8> {
        self.user_id_map.read().await.get(&user_id).map(|x| *x)
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

    pub(super) fn spawn(self) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(super::CHANNEL_BOUND);
        tokio::spawn(self.run(rx));
        tx
    }

    pub(super) async fn run(self, mut recv: tokio::sync::mpsc::Receiver<super::GameMessage>) {
        while !recv.is_closed() {
            if let Some(msg) = recv.recv().await {
                match msg {
                    super::GameMessage::NewConnection { user, game_guid, connection, response, sender } => {
                        if self.game_guid != game_guid {
                            log::error!("Game guid does not match (got: {}, expected: {})", game_guid, self.game_guid);
                            response.send(Some(super::messages::ErrorMessage {
                                message: format!("Game guid does not match (got: {}, expected: {})", game_guid, self.game_guid),
                                inner: None,
                            })).unwrap_or_default();
                            return;
                        } else {
                            let mut users = self.users.write().await;
                            let new_user = UserConnection {
                                user,
                                connection: UserSender {
                                    connection,
                                    sender,
                                },
                                state: std::sync::Arc::new(UserState::new()),
                                machine: MachineState::new(),
                            };
                            //tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            //let id = users.len() as u8;
                            match new_user.user.game_players(&game_guid).await {
                                Ok(players) => {
                                    if self.player_count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                                        self.player_count.store(players.len() as _, std::sync::atomic::Ordering::Relaxed);
                                    }
                                    let user_id = new_user.user.user_id();
                                    let id = players.iter().filter(|p| p.user_id == user_id).next().map(|p| p.player_id).unwrap();
                                    self.spawn_send_loading_events(&new_user, id, players);
                                    log::debug!("User {} is validated to play game {}", new_user.user.user_id(), game_guid);
                                    self.user_id_map.write().await.insert(new_user.user.user_id(), id);
                                    users.insert(id, new_user);
                                    response.send(None).unwrap_or_default();
                                },
                                Err(e) => {
                                    log::error!("Failed to retrieve players for game {}: {}", game_guid, e);
                                    response.send(Some(super::messages::ErrorMessage {
                                        message: "Failed to retrieve players for game".to_owned(),
                                        inner: Some(Box::new(e)),
                                    })).unwrap_or_default();
                                }
                            }

                        }
                    },
                    super::GameMessage::LoadingProgress { user_id, user_name, progress } => {
                        let progress_data = rlnl::events::loading::LoadingProgress {
                            user_name: rlnl::types::BinaryWriterString(user_name),
                            progress,
                        };
                        let mut all_users_loading_complete = true;
                        for conn in self.users.read().await.values() {
                            if user_id == conn.user.user_id() {
                                let progress_percent = ((progress * 100.0).ceil() as u8).clamp(0, 100);
                                log::debug!("User {} is loaded {}% into game {}", user_id, progress_percent, self.game_guid);
                                conn.state.progress.store(progress_percent, std::sync::atomic::Ordering::Relaxed);
                                if progress_percent != 100 {
                                    all_users_loading_complete = false;
                                }
                            } else {
                                all_users_loading_complete &= conn.state.progress.load(std::sync::atomic::Ordering::Relaxed) == 100;
                            }
                            let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                            match mode {
                                ConnectionMode::Loading | ConnectionMode::WaitingForSync => {},
                                ConnectionMode::Sync | ConnectionMode::WaitingToStart => {
                                    if user_id != conn.user.user_id() {
                                        crate::events::log_lnl_send_failure(conn.connection.rlnl()
                                            .send_data(&progress_data, rlnl::event_code::NetworkEvent::BroadcastLoadingProgress, literustlib::packet::Property::ReliableOrdered, &conn.connection.connection).await);
                                    }
                                    /*if progress > 0.95 {
                                        log::info!("User {} is ready, ending sync", user_id);
                                        crate::events::log_lnl_send_failure(crate::handlers::simple_typed::RlnlSender::new(&conn.sender)
                                            .send_empty(
                                                rlnl::event_code::NetworkEvent::EndOfSync,
                                                literustlib::packet::Property::ReliableOrdered,
                                                &conn.connection,
                                            )
                                        .await);
                                        conn.mode.store(ConnectionMode::InGame.to_u8(), std::sync::atomic::Ordering::Relaxed);
                                    }*/
                                },
                                ConnectionMode::InGame => {
                                    log::warn!("Got loading progress for user {} who is supposed to be already in-game", user_id);
                                },
                            }
                        }
                        if all_users_loading_complete {
                            for (id, conn) in self.users.read().await.iter() {
                                if let Err(e) = conn.connection.rlnl().send_empty(
                                    rlnl::event_code::NetworkEvent::EndOfSync,
                                    literustlib::packet::Property::ReliableOrdered,
                                    &conn.connection.connection
                                ).await {
                                    log::error!("Failed to send EndOfSync event to user {}: {}", id, e);
                                }
                            }
                        }
                    }
                    super::GameMessage::RequestLoadingProgress { user_id } => {
                        let mut user_info = None;
                        for conn in self.users.read().await.values() {
                            if user_id == conn.user.user_id() {
                                user_info = Some((
                                    conn.connection.to_owned(),
                                    rlnl::events::loading::LoadingProgress {
                                        user_name: rlnl::types::BinaryWriterString(conn.user.user_name().to_owned()),
                                        progress: (conn.state.progress.load(std::sync::atomic::Ordering::Relaxed) as f32) / 100.0,
                                    },
                                ));
                            }
                        }
                        if let Some(user_info) = user_info {
                            let sender = crate::handlers::RlnlSender::new(&user_info.0.sender);
                            for conn in self.users.read().await.values() {
                                if user_id == conn.user.user_id() { continue; }
                                crate::events::log_lnl_send_failure(sender.send_data(
                                    &user_info.1,
                                    rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                                    literustlib::packet::Property::ReliableOrdered,
                                    &user_info.0.connection,
                                ).await);
                            }
                        } else {
                            log::error!("Failed to find user {} in connected users for match {}", user_id, self.game_guid);
                        }

                    },
                    super::GameMessage::WeaponSelect { user_id, machine_id, category, size } => {
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
                                false
                            ).await;
                        }
                    },
                    super::GameMessage::RequestLoadingSync { user_id } => {
                        // wait for all users to be ready before transitioning to loading sync
                        let mut ready_count = 0;
                        for user in self.users.read().await.values() {
                            if user.user.user_id() == user_id {
                                user.state.mode.store(ConnectionMode::WaitingForSync.to_u8(), std::sync::atomic::Ordering::Relaxed);
                                ready_count += 1;
                            } else {
                                if matches!(ConnectionMode::from_u8(user.state.mode.load(std::sync::atomic::Ordering::Relaxed)), ConnectionMode::WaitingForSync) {
                                    ready_count += 1;
                                }
                            }
                        }
                        let player_count = self.player_count.load(std::sync::atomic::Ordering::Relaxed) as usize;
                        if ready_count == player_count {
                            log::info!("All players ({}) awaiting sync for game {}", player_count, self.game_guid);
                            let total_users = self.users.read().await.len() as u8;
                            for (user_key, conn) in self.users.read().await.iter() {
                                self.spawn_send_sync_events(conn, conn.user.user_id(), *user_key, total_users);
                            }
                        }
                    },
                    super::GameMessage::LoadComplete { user_id } => {
                        if let Some(user_key) = self.user_key_by_user_id(user_id).await {
                            if let Some(conn) = self.users.read().await.get(&user_key) {
                                log::info!("Loading complete for game {}, user {} ({})", self.game_guid, user_id, user_key);
                                conn.state.progress.store(100, std::sync::atomic::Ordering::Relaxed);
                                conn.state.mode.store(ConnectionMode::WaitingToStart.to_u8(), std::sync::atomic::Ordering::Relaxed);
                                /*let game_start = chrono::DateTime::from_timestamp(self.game_start.load(std::sync::atomic::Ordering::Relaxed), 0).unwrap();
                                let payload = super::countdown::time_to_game_start_payload(game_start);
                                let sender = conn.connection.rlnl();
                                if let Err(e) = sender.send_data(
                                    &payload,
                                    rlnl::event_code::NetworkEvent::TimeToGameStart,
                                    literustlib::packet::Property::ReliableOrdered,
                                    &conn.connection.connection)
                                .await {
                                    log::error!("Failed to send updated TimeToGameStart to a user: {}", e);
                                }*/
                                self.spawn_initial_ingame_events(conn, user_id);
                            } else {
                                log::warn!("Invalid LoadComplete user key {} for game {}", user_key, self.game_guid);
                                continue;
                            }
                        } else {
                            log::warn!("Unknown LoadComplete user id {} for game {}", user_id, self.game_guid);
                            continue;
                        }
                        // wait for all users to be ready for starting game start countdown
                        let mut all_users_loading_complete = true;
                        for conn in self.users.read().await.values() {
                            let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                            all_users_loading_complete &= matches!(mode, ConnectionMode::WaitingToStart);
                        }
                        // trigger game start
                        if all_users_loading_complete {
                            let player_count = self.player_count.load(std::sync::atomic::Ordering::Relaxed) as usize;
                            log::info!("All players ({}) are ready for game {}", player_count, self.game_guid);
                            tokio::time::sleep(Self::END_OF_SYNC_DELAY).await;
                            let mut senders = Vec::new();
                            for conn in self.users.read().await.values() {
                                senders.push((conn.connection.clone(), conn.state.clone()));
                            }
                            let game_start = chrono::Utc::now() + Self::COUNTDOWN_DURATION;
                            self.game_start.store(game_start.timestamp(), std::sync::atomic::Ordering::Relaxed);
                            super::countdown::match_countdown(senders, game_start);
                        }
                    }
                    super::GameMessage::BroadcastRlnl { user_id: _, event, property, data } => {
                        if let Some(data) = data {
                            self.broadcast(event, property, &*data, true).await;
                        } else {
                            self.broadcast_dataless(event, property, true).await;
                        }
                    }
                    super::GameMessage::RebroadcastRlnl { skip_user_id, event, property, data } => {
                        if let Some(data) = data {
                            self.rebroadcast(skip_user_id, event, property, &*data, true).await;
                        } else {
                            self.rebroadcast_dataless(skip_user_id, event, property, true).await;
                        }

                    }
                    super::GameMessage::Motion { user_id, data } => {
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
                    super::GameMessage::NoOp => {},
                }
            }
        }
        self.is_complete.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn spawn_send_loading_events(&self, user: &UserConnection, player_id: u8, players: Vec<oj_rc_core::persist::user::PlayerDescriptor>) {
        let connection = user.connection.clone();
        let user_id = user.user.user_id();
        tokio::spawn(Self::send_loading_events_wrapper(connection, player_id, user_id, players));
    }

    async fn send_loading_events_wrapper(connection: UserSender, player_id: u8, user_id: i32, players: Vec<oj_rc_core::persist::user::PlayerDescriptor>) {
        if let Err(e) = Self::send_loading_events(&connection, player_id, players).await {
            log::error!("Failed to send Loading events for user {} ({}): {}", user_id, player_id, e);
        }
    }

    async fn send_loading_events(user: &UserSender, player_id: u8, players: Vec<oj_rc_core::persist::user::PlayerDescriptor>) -> std::io::Result<()> {
        let sender = user.rlnl();
        sender.send_data(
            &rlnl::events::ingame::PlayerId { player: player_id },
            rlnl::event_code::NetworkEvent::GameGuidValidated,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection
        ).await?;
        sender.send_data(
            &rlnl::events::loading::PlayerIDsAndNames {
                num_players: players.len() as _,
                players: players.into_iter().map(|player| rlnl::events::loading::PlayerIDAndName {
                    player_id: player.player_id as _,
                    name: rlnl::types::BinaryWriterString(player.public_id),
                    display_name: rlnl::types::BinaryWriterString(player.display_name),
                })
                .collect(),
                /*players: vec![ // FIXME
                    rlnl::events::loading::PlayerIDAndName {
                        player_id: 0,
                        name: rlnl::types::BinaryWriterString("NGniusness".to_owned()),
                        display_name: rlnl::types::BinaryWriterString("NGniusness".to_owned()),
                    },
                    rlnl::events::loading::PlayerIDAndName {
                        player_id: 1,
                        name: rlnl::types::BinaryWriterString("NGniusness2".to_owned()),
                        display_name: rlnl::types::BinaryWriterString("NGniusness2".to_owned()),
                    },
                ],*/
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

    fn spawn_send_sync_events(&self, user: &UserConnection, user_id: i32, player_id: u8, num_players: u8) {
        let connection = user.connection.clone();
        tokio::spawn(Self::send_sync_events_wrapper(connection, user_id, player_id, num_players));
        user.state.mode.store(ConnectionMode::Sync.to_u8(), std::sync::atomic::Ordering::Relaxed);
    }

    async fn send_sync_events_wrapper(connection: UserSender, user_id: i32, player_id: u8, num_players: u8) {
        if let Err(e) = Self::send_sync_events(connection, player_id, num_players).await {
            log::error!("Failed to send Sync events for user {}: {}", user_id, e);
        }
    }

    async fn send_sync_events(connection: UserSender, _player_id: u8, num_players: u8) -> std::io::Result<()> {
        let sender = connection.rlnl();
        sender.send_empty(
            rlnl::event_code::NetworkEvent::BeginSync,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;
        // sudden death
        sender.send_data(
            &rlnl::events::sync::UpdateGameModeSettings { // FIXME use value from config
                respawn_heal_duration: 10.0,
                respawn_full_heal_duration: 10.0,
            },
            rlnl::event_code::NetworkEvent::GameModeSettings,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;
        sender.send_data(
            &rlnl::events::GameTime(300.0), // FIXME use value from config
            rlnl::event_code::NetworkEvent::CurrentGameTime,
            literustlib::packet::Property::ReliableOrdered,
            &connection.connection)
        .await?;
        // generic
        sender.send_data(
            &rlnl::events::sync::InitialiseGameStats {
                num_players,
                stats: (0..num_players).into_iter()
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
        for i in 0..num_players {
            sender.send_data(
                &rlnl::events::sync::SpawnPoint {
                    pos: rlnl::types::PosQuatPair {
                        pos: rlnl::types::CompressedVec3 { x: i as _, y: 42, z: i as _ },
                        rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                    },
                    owner: i,
                },
                rlnl::event_code::NetworkEvent::FreeSpawnPoint,
                literustlib::packet::Property::ReliableOrdered,
                &connection.connection)
            .await?;
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
}

impl super::GamemodeEngine for GenericGamemodeEngine {}
