use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy)]
pub enum GamemodeChangeStrategy {
    Upgrade, // move enqueued players into newer gamemode
    Notify, // send match change
    Ignore, // do nothing
}

impl GamemodeChangeStrategy {
    fn from_core(core: oj_rc_core::persist::config::QueueChangeMode) -> Self {
        match core {
            oj_rc_core::persist::config::QueueChangeMode::Upgrade => Self::Upgrade,
            oj_rc_core::persist::config::QueueChangeMode::Notify => Self::Notify,
            oj_rc_core::persist::config::QueueChangeMode::Ignore => Self::Ignore,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct QueueKey {
    map: String,
    mode: oj_rc_core::data::game_mode::GameMode,
    visibility: oj_rc_core::data::game_mode::MapVisibility,
    auto_heal: bool,
}

struct QueueUser {
    emitter: polariton_server::events::EventEmitter,
    player: oj_rc_core::data::player_data::PlayerData,
    user_id: i32,
}

pub struct QueueHandler {
    users_in_queue: tokio::sync::Mutex<HashMap<QueueKey, Vec<QueueUser>>>,
    users_per_game: usize,
    is_enabled: bool,
    hostname: String,
    hostport: u16,
    network_conf: crate::data::network::NetworkConfigData,
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
    weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    change_strategy: GamemodeChangeStrategy,
}

impl QueueHandler {
    pub fn new(conf: &oj_rc_core::ConfigImpl, game_host: &str, factory: std::sync::Arc<oj_rc_core::factory::Factory>, cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>, weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,) -> Self {
        let (domain, port_str) = game_host.split_once(':').expect("Invalid redirect address (must be domain:port)");
        Self {
            users_in_queue: tokio::sync::Mutex::new(HashMap::new()),
            users_per_game: oj_rc_core::ConfigProvider::<()>::players_per_game(conf),
            is_enabled: oj_rc_core::ConfigProvider::<()>::is_multiplayer_enabled(conf),
            hostname: domain.to_owned(),
            hostport: port_str.parse().expect("Invalid redirect port"),
            network_conf: crate::data::network::NetworkConfigData::from_conf(oj_rc_core::ConfigProvider::<()>::network_config(conf)),
            factory,
            cpu_counter,
            weapon_guesser,
            change_strategy: GamemodeChangeStrategy::from_core(<oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::server_config(conf).queue_mode),
        }
    }

    async fn enter_match(&self, key: QueueKey, mut players: Vec<QueueUser>, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync)) {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        chrono::Utc::now().timestamp_micros().hash(&mut hasher);
        let guid = oj_rc_core::persist::user::uuid_sanitize(hasher.finish() as i64);
        let guid_str = oj_rc_core::persist::user::i64_as_uuid_str(guid);
        let team_picker = match key.mode {
            oj_rc_core::data::game_mode::GameMode::Pit => |i| i as i32, // each player is on a different team
            _ => |i| (i % 2) as i32, // alternate teams
        };
        for (i, player) in players.iter_mut().enumerate() {
            player.player.team = team_picker(i);
        }
        let player_descs = players.iter().map(|x| oj_rc_core::persist::user::PlayerLobbyDescriptor {
            user_id: x.user_id,
            team: x.player.team,
            group: None, // TODO support platoons
            public_id: x.player.name.clone(),
            display_name: x.player.display_name.clone(),
        }).collect();
        let game_desc = oj_rc_core::persist::user::GameDescriptor {
            guid: guid_str.clone(),
            map: key.map.clone(),
            mode: key.mode,
            visibility: key.visibility,
            auto_heal: key.auto_heal,
            is_ranked: false,
            is_custom: false,
            is_complete: false,
        };
        match user.start_game(game_desc, player_descs, self.factory.as_ref(), &self.cpu_counter, &self.weapon_guesser).await {
            Ok(fakes) => {
                let player_datas = players.iter().map(|x| x.player.clone())
                    .chain(fakes.players.into_iter().map(|(desc, _emu)| desc))
                    .collect();
                let enter_battle_ev = crate::events::battle_enter::BattleEnter {
                    host: self.hostname.clone(),
                    port: self.hostport,
                    map: key.map.clone(),
                    mode: key.mode,
                    guid: guid_str,
                    is_ranked: false,
                    is_custom: false,
                    visibility: Some(key.visibility),
                    auto_heal: key.auto_heal,
                    player_datas,
                    network_config: self.network_conf.clone(),
                };
                let arc_event = std::sync::Arc::new(enter_battle_ev);
                for player in players.iter() {
                    tokio::spawn(Self::send_events_to_player(arc_event.clone(), player.emitter.clone()));
                }
            },
            Err(e) => {
                if let Some(msg) = e.error_msg() {
                    log::error!("Cannot send enter battle events to players since LobbyUser.start_game(...) failed: {} ({})", msg, e.error_code());
                } else {
                    log::error!("Cannot send enter battle events to players since LobbyUser.start_game(...) failed ({})", e.error_code());
                }

            }
        }

    }

    async fn send_events_to_player(enter_event: std::sync::Arc<crate::events::battle_enter::BattleEnter>, sender: polariton_server::events::EventEmitter) {
        const WAIT_BEFORE_ENTER: std::time::Duration = std::time::Duration::from_secs(1);
        if sender.emit(crate::events::battle_found::BattleFound) {
            tokio::time::sleep(WAIT_BEFORE_ENTER).await;
            sender.emit(enter_event.as_ref());
        }
    }

    pub async fn join_queue(&self, map: String, mode: oj_rc_core::data::game_mode::GameMode, visibility: oj_rc_core::data::game_mode::MapVisibility, auto_heal: bool, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync), event_emitter: polariton_server::events::EventEmitter) {
        if !self.is_enabled {
            event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                code: oj_rc_core::data::error_codes::LobbyReasonCode::NoSuitableLobbyFound as i16,
                text: "Multiplayer is not enabled".to_owned(),
            });
            return;
        }
        let key = QueueKey {
            map, mode, visibility, auto_heal,
        };
        match user.player_data(&self.cpu_counter).await {
            Ok(player_data) => {
                let new_player = QueueUser {
                    emitter: event_emitter,
                    player: player_data,
                    user_id: user.user_id(),
                };
                let mut lock = self.users_in_queue.lock().await;
                // handle game event change
                if !lock.contains_key(&key) && lock.len() == 1 {
                    log::debug!("Game event change detected");
                    match self.change_strategy {
                        GamemodeChangeStrategy::Upgrade => {
                            let mut new_queue_map = std::collections::HashMap::<QueueKey, Vec<QueueUser>>::with_capacity(lock.len());
                            let mut count = 0;
                            for (_key, mut users) in lock.drain() {
                                count += users.len();
                                if let Some(values) = new_queue_map.get_mut(&key) {
                                    values.append(&mut users);
                                } else {
                                    new_queue_map.insert(key.clone(), users);
                                }

                            }
                            *lock = new_queue_map;
                            if count != 0 {
                                log::info!("Upgraded {} users in queue to new gamemode", count);
                            }
                        },
                        GamemodeChangeStrategy::Notify => {
                            let mut count = 0;
                            for (_key, users) in lock.drain() {
                                count += users.len();
                                for player in users {
                                    player.emitter.emit(crate::events::enqueue_error::QueueJoinError {
                                        code: oj_rc_core::data::error_codes::LobbyReasonCode::EventSystemExpired as i16,
                                        text: "Please requeue".to_owned(),
                                    });
                                }
                            }
                            if count != 0 {
                                log::info!("Notified {} users in queue of new gamemode", count);
                            }
                        },
                        GamemodeChangeStrategy::Ignore => {
                            log::debug!("Gamemode appears to have changed, ignoring already-queued players");
                        }
                    }
                }
                let players_len = if let Some(players) = lock.get_mut(&key) {
                    players.push(new_player);
                    players.len()
                } else {
                    lock.insert(key.clone(), vec![new_player]);
                    1
                };
                let game_ready = players_len >= self.users_per_game;
                let players = if game_ready { lock.remove(&key) } else { None };
                drop(lock);
                if let Some(players) = players {
                    self.enter_match(key, players, user).await;
                }
            },
            Err(e) => {
                event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                    code: e.error_code(),
                    text: e.error_msg().map(|x| x.to_owned()).unwrap_or_else(|| "Unknown queue join error".to_owned()),
                });
            }
        }
    }

    pub async fn leave_queue(&self, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync)) {
        let user_id = user.user_id();
        for queue in self.users_in_queue.lock().await.values_mut() {
            if let Some((i, _)) = queue.iter().enumerate().find(|(_, user)| user.user_id == user_id) {
                queue.remove(i);
                log::info!("User {} was removed from a queue", user_id);
                break;
            }
        }
    }
}
