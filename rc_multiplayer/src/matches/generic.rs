pub(super) struct UserConnection {
    pub(super) user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
    pub(super) connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
    pub(super) sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>,
    pub(super) state: UserState,
    pub(super) machine: MachineState,
}

pub(super) struct UserState {
    pub(super) mode: std::sync::atomic::AtomicU8,
    pub(super) progress: std::sync::atomic::AtomicU8, // percent
    _x: (),
}

impl UserState {
    fn new() -> Self {
        Self {
            mode: std::sync::atomic::AtomicU8::new(ConnectionMode::Loading.to_u8()),
            progress: std::sync::atomic::AtomicU8::new(0),
            _x: (),
        }
    }
}

pub(super) struct MachineState {
    pub(super) selected_weapon: WeaponInfo,
    _x: (),
}

impl MachineState {
    fn new() -> Self {
        Self {
            selected_weapon: WeaponInfo::new(),
            _x: (),
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
    Sync = 1,
    InGame = 2,
}

impl ConnectionMode {
    #[inline]
    fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::Loading,
            1 => Self::Sync,
            2 => Self::InGame,
            x => panic!("Unrecognized ConnectionMode {}", x),
        }
    }

    #[inline]
    fn to_u8(self) -> u8 {
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
}

impl GenericGamemodeEngine {
    pub fn new(guid: String) -> Self {

        Self {
            users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            user_id_map: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            game_guid: guid,
            is_complete: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub(super) async fn user_key_by_user_id(&self, user_id: i32) -> Option<u8> {
        self.user_id_map.read().await.get(&user_id).map(|x| *x)
    }

    pub(super) async fn broadcast<T: byteserde::ser_heap::ByteSerializeHeap>(&self, user_id: i32, code: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: T) {
        for conn in self.users.read().await.values() {
            if user_id == conn.user.user_id() { continue; }
            let sender = crate::handlers::simple_typed::RlnlSender::new(&conn.sender);
            crate::events::log_lnl_send_failure(sender.send_data(
                &data,
                code,
                property,
                &conn.connection,
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
                            response.send(Some(super::messages::ErrorMessage {
                                message: "Game guid does not match".to_owned(),
                                inner: None,
                            })).unwrap_or_default();
                            return;
                        } else {
                            let mut users = self.users.write().await;
                            let new_user = UserConnection {
                                user,
                                connection,
                                sender,
                                state: UserState::new(),
                                machine: MachineState::new(),
                            };
                            //tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            let id = users.len() as u8;
                            if let Err(e) = self.send_loading_events(&new_user, id).await {
                                response.send(Some(super::messages::ErrorMessage {
                                    message: "Failed to send GameGuidValidated response".to_owned(),
                                    inner: Some(Box::new(e)),
                                })).unwrap_or_default();
                                return;
                            }
                            log::debug!("User {} is validated to play game {}", new_user.user.user_id(), game_guid);
                            self.user_id_map.write().await.insert(new_user.user.user_id(), id);
                            users.insert(id, new_user);
                            response.send(None).unwrap_or_default();
                        }
                    },
                    super::GameMessage::LoadingProgress { user_id, user_name, progress } => {
                        let progress_data = rlnl::events::loading::LoadingProgress {
                            user_name: rlnl::types::BinaryWriterString(user_name),
                            progress,
                        };
                        for conn in self.users.read().await.values() {
                            if user_id == conn.user.user_id() {
                                let progress_percent = (progress * 100.0).ceil() as u8;
                                log::debug!("User {} is loaded {}% into game {}", user_id, progress_percent, self.game_guid);
                                conn.state.progress.store(progress_percent, std::sync::atomic::Ordering::Relaxed);
                            }
                            let mode = ConnectionMode::from_u8(conn.state.mode.load(std::sync::atomic::Ordering::Relaxed));
                            match mode {
                                ConnectionMode::Loading
                                | ConnectionMode::Sync => {
                                    if user_id != conn.user.user_id() {
                                        crate::events::log_lnl_send_failure(crate::handlers::simple_typed::RlnlSender::new(&conn.sender)
                                            .send_data(&progress_data, rlnl::event_code::NetworkEvent::BroadcastLoadingProgress, literustlib::packet::Property::ReliableOrdered, &conn.connection).await);
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
                    }
                    super::GameMessage::RequestLoadingProgress { user_id } => {
                        let mut user_info = None;
                        for conn in self.users.read().await.values() {
                            if user_id == conn.user.user_id() {
                                user_info = Some((
                                    conn.sender.to_owned(),
                                    conn.connection.to_owned(),
                                    rlnl::events::loading::LoadingProgress {
                                        user_name: rlnl::types::BinaryWriterString(conn.user.user_name().to_owned()),
                                        progress: (conn.state.progress.load(std::sync::atomic::Ordering::Relaxed) as f32) / 100.0,
                                    },
                                ));
                            }
                        }
                        if let Some(user_info) = user_info {
                            let sender = crate::handlers::simple_typed::RlnlSender::new(&user_info.0);
                            for conn in self.users.read().await.values() {
                                if user_id == conn.user.user_id() { continue; }
                                crate::events::log_lnl_send_failure(sender.send_data(
                                    &user_info.2,
                                    rlnl::event_code::NetworkEvent::BroadcastLoadingProgress,
                                    literustlib::packet::Property::ReliableOrdered,
                                    &user_info.1,
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
                            self.broadcast(
                                user_id,
                                rlnl::event_code::NetworkEvent::BroadcastWeaponSelect,
                                literustlib::packet::Property::ReliableOrdered,
                                data,
                            ).await;
                        }
                    },
                    super::GameMessage::RequestLoadingSync { user_id } => {
                        if let Some(user_key) = self.user_key_by_user_id(user_id).await {
                            if let Some(conn) = self.users.read().await.get(&user_key) {
                                self.spawn_send_sync_events(conn, user_id);
                            }
                        }
                    },
                    super::GameMessage::Motion { user_id, data } => {
                        for conn in self.users.read().await.values() {
                            if conn.user.user_id() == user_id { continue; } // fun fact: the game hard crashes if you omit this
                            crate::events::log_lnl_send_failure(conn.sender.send_data(crate::handler::EventData {
                                message_ty: crate::data::MessageType::RobotMotion,
                                variant: 0,
                                data_size: data.len() as _,
                                data: data.clone(),
                            }, literustlib::packet::Property::Unreliable, &conn.connection).await);
                        }
                    }
                    super::GameMessage::NoOp => {},
                }
            }
        }
        self.is_complete.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    async fn send_loading_events(&self, user: &UserConnection, player_id: u8) -> std::io::Result<()> {
        let sender = crate::handlers::simple_typed::RlnlSender::new(&user.sender);
        sender.send_data(
            &rlnl::events::loading::PlayerID { owner: player_id },
            rlnl::event_code::NetworkEvent::GameGuidValidated,
            literustlib::packet::Property::ReliableOrdered,
            &user.connection
        ).await?;
        sender.send_data(
            &rlnl::events::loading::PlayerIDsAndNames {
                num_players: 2,
                players: vec![ // FIXME
                    rlnl::events::loading::PlayerIDAndName {
                        player_id: 0,
                        name: rlnl::types::BinaryWriterString("NGniusness".to_owned()),
                        display_name: rlnl::types::BinaryWriterString("NGniusness".to_owned()),
                    },
                    rlnl::events::loading::PlayerIDAndName {
                        player_id: 1,
                        name: rlnl::types::BinaryWriterString("NGniusness_echo".to_owned()),
                        display_name: rlnl::types::BinaryWriterString("NGniusness_echo".to_owned()),
                    },
                ],
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

    fn spawn_send_sync_events(&self, user: &UserConnection, user_id: i32) {
        let sender = user.sender.clone();
        let connection = user.connection.clone();
        tokio::spawn(Self::send_sync_events_wrapper(connection, sender, user_id));
        user.state.mode.store(ConnectionMode::Sync.to_u8(), std::sync::atomic::Ordering::Relaxed);
    }

    async fn send_sync_events_wrapper(connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>, user_id: i32) {
        if let Err(e) = Self::send_sync_events(connection, sender).await {
            log::error!("Failed to send Sync events for user {}: {}", user_id, e);
        }
    }

    async fn send_sync_events(connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) -> std::io::Result<()> {
        let sender = crate::handlers::simple_typed::RlnlSender::new(&sender);
        sender.send_empty(
            rlnl::event_code::NetworkEvent::BeginSync,
            literustlib::packet::Property::ReliableOrdered,
            &connection)
        .await?;
        // sudden death
        sender.send_data(
            &rlnl::events::sync::UpdateGameModeSettings { // FIXME use value from config
                respawn_heal_duration: 10.0,
                respawn_full_heal_duration: 10.0,
            },
            rlnl::event_code::NetworkEvent::GameModeSettings,
            literustlib::packet::Property::ReliableOrdered,
            &connection)
        .await?;
        sender.send_data(
            &rlnl::events::GameTime(300.0), // FIXME use value from config
            rlnl::event_code::NetworkEvent::CurrentGameTime,
            literustlib::packet::Property::ReliableOrdered,
            &connection)
        .await?;
        // generic
        sender.send_data(
            &rlnl::events::sync::InitialiseGameStats {
                num_players: 2,
                stats: vec![ // FIXME generate one per connection
                    rlnl::types::IngamePlayerStats {
                        player_name: 0,
                        num_stats: 0,
                        stats: vec![],
                    },
                    rlnl::types::IngamePlayerStats {
                        player_name: 1,
                        num_stats: 0,
                        stats: vec![],
                    },
                ],
            },
            rlnl::event_code::NetworkEvent::InitialiseGameStats,
            literustlib::packet::Property::ReliableOrdered,
            &connection)
        .await?;
        sender.send_data(
            &rlnl::events::sync::SpawnPoint {
                pos: rlnl::types::PosQuatPair {
                    pos: rlnl::types::CompressedVec3 { x: 0, y: 0, z: 0 },
                    rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                },
                owner: 0,
            },
            rlnl::event_code::NetworkEvent::FreeSpawnPoint,
            literustlib::packet::Property::ReliableOrdered,
            &connection)
        .await?;
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
}

impl super::GamemodeEngine for GenericGamemodeEngine {
    fn is_complete(&self) -> bool {
        self.is_complete.load(std::sync::atomic::Ordering::Relaxed) // for now, this is never closed
    }
}
