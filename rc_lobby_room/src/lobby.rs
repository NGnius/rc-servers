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

impl QueueKey {
    fn display_guid(&self) -> String {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        //chrono::Utc::now().timestamp_micros().hash(&mut hasher);
        let guid = oj_rc_core::persist::user::uuid_sanitize(hasher.finish() as i64);
        oj_rc_core::persist::user::i64_as_uuid_str(guid)
    }

    fn unique_guid(&self) -> String {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        chrono::Utc::now().timestamp_micros().hash(&mut hasher);
        let guid = oj_rc_core::persist::user::uuid_sanitize(hasher.finish() as i64);
        oj_rc_core::persist::user::i64_as_uuid_str(guid)
    }
}

struct QueueUser {
    emitter: polariton_server::events::EventEmitter,
    player: oj_rc_core::data::player_data::PlayerData,
    user_id: i32,
    enqueued_at: chrono::DateTime<chrono::Utc>,
    user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
}

pub struct QueueHandler {
    users_in_queue: std::sync::Arc<tokio::sync::Mutex<HashMap<QueueKey, Vec<QueueUser>>>>,
    users_per_game: usize,
    is_enabled: bool,
    hostname: String,
    hostport: u16,
    network_conf: crate::data::network::NetworkConfigData,
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
    weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    change_strategy: GamemodeChangeStrategy,
    autostart_after: Option<std::time::Duration>,
    autostart_task_started: std::sync::atomic::AtomicBool,
}

impl QueueHandler {
    pub fn new(conf: &oj_rc_core::ConfigImpl, game_host: &str, factory: std::sync::Arc<oj_rc_core::factory::Factory>, cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>, weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,) -> Self {
        let (domain, port_str) = game_host.split_once(':').expect("Invalid redirect address (must be domain:port)");
        let mp_settings = oj_rc_core::ConfigProvider::<()>::multiplayer_settings(conf);
        Self {
            users_in_queue: std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            users_per_game: oj_rc_core::ConfigProvider::<()>::players_per_game(conf),
            is_enabled: mp_settings.is_enabled,
            hostname: domain.to_owned(),
            hostport: port_str.parse().expect("Invalid redirect port"),
            network_conf: crate::data::network::NetworkConfigData::from_conf(oj_rc_core::ConfigProvider::<()>::network_config(conf)),
            factory,
            cpu_counter,
            weapon_guesser,
            change_strategy: GamemodeChangeStrategy::from_core(<oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::server_config(conf).queue_mode),
            autostart_after: mp_settings.lobby_autostart_after,
            autostart_task_started: std::sync::atomic::AtomicBool::new(false),
        }
    }

    fn ensure_autostart_task_running(&self) {
        if self.autostart_task_started.swap(true, std::sync::atomic::Ordering::AcqRel) || self.autostart_after.is_none() {
            return;
        }

        let users_in_queue = self.users_in_queue.clone();
        let users_per_game = self.users_per_game;
        let hostname = self.hostname.clone();
        let hostport = self.hostport;
        let network_conf = self.network_conf.clone();
        let factory = self.factory.clone();
        let cpu_counter = self.cpu_counter.clone();
        let weapon_guesser = self.weapon_guesser.clone();
        let autostart_after = self.autostart_after.unwrap();

        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now();
                let mut to_start: Vec<(QueueKey, Vec<QueueUser>)> = Vec::new();
                let mut next_deadline: Option<chrono::DateTime<chrono::Utc>> = None;

                {
                    let mut lock = users_in_queue.lock().await;

                    let mut empty_keys: Vec<QueueKey> = Vec::new();
                    let mut expired_keys: Vec<QueueKey> = Vec::new();

                    for (key, users) in lock.iter() {
                        if users.is_empty() {
                            empty_keys.push(key.clone());
                            continue;
                        }

                        // first user is always oldest within a queue
                        let deadline = users[0].enqueued_at + autostart_after;
                        if now >= deadline {
                            expired_keys.push(key.clone());
                        } else {
                            next_deadline = Some(match next_deadline {
                                Some(d) => if deadline < d { deadline } else { d },
                                None => deadline,
                            });
                        }
                    }

                    for k in empty_keys {
                        lock.remove(&k);
                    }

                    for k in expired_keys {
                        if let Some(players) = lock.remove(&k) {
                            if !players.is_empty() {
                                to_start.push((k, players));
                            }
                        }
                    }
                }

                // start expired queues outside the lock
                for (key, players) in to_start {
                    // choose the oldest queued user's LobbyUser handle
                    let starter = match players.first() {
                        Some(p) => p.user.clone(),
                        None => continue,
                    };

                    QueueHandler::enter_match_static(
                        hostname.clone(), 
                        hostport, 
                        network_conf.clone(), 
                        factory.clone(), 
                        cpu_counter.clone(), 
                        weapon_guesser.clone(), 
                        users_per_game, 
                        key, 
                        players, 
                        starter.as_ref().as_ref(),
                    ).await;
                }

                let sleep_dur = if let Some(deadline) = next_deadline {
                    let now2 = chrono::Utc::now();
                    match (deadline - now2).to_std() {
                        Ok(d) => d,
                        Err(_) => std::time::Duration::from_secs(0),
                    }
                } else {
                    std::time::Duration::from_secs(1)
                };

                tokio::time::sleep(sleep_dur).await;
            }
        });
    }

    async fn enter_match(&self, key: QueueKey, players: Vec<QueueUser>, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync)) {
        Self::enter_match_static(
            self.hostname.clone(),
            self.hostport,
            self.network_conf.clone(),
            self.factory.clone(),
            self.cpu_counter.clone(),
            self.weapon_guesser.clone(),
            self.users_per_game,
            key,
            players,
            user,
        ).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn enter_match_static(hostname: String, hostport: u16, network_conf: crate::data::network::NetworkConfigData, factory: std::sync::Arc<oj_rc_core::factory::Factory>, cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>, weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>, users_per_game: usize, key: QueueKey, mut players: Vec<QueueUser>, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync)) {
        let guid_str = key.unique_guid();
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
        let team_picker = user.team_chooser(&game_desc).await;
        /*let team_picker = match key.mode {
            oj_rc_core::data::game_mode::GameMode::Pit => |i| i as i32, // each player is on a different team
            _ => |i| (i % 2) as i32, // alternate teams
        };*/
        for (i, player) in players.iter_mut().enumerate() {
            player.player.team = team_picker.team(i);
        }
        let player_descs = players.iter().map(|x| oj_rc_core::persist::user::PlayerLobbyDescriptor {
            user_id: x.user_id,
            team: x.player.team,
            group: None, // TODO support platoons
            public_id: x.player.name.clone(),
            display_name: x.player.display_name.clone(),
        }).collect();

        let missing = users_per_game.saturating_sub(players.len());

        match user.start_game(game_desc, player_descs, factory.as_ref(), &cpu_counter, &weapon_guesser, &team_picker, missing).await {
            Ok(fakes) => {
                let player_datas = players.iter().map(|x| x.player.clone())
                    .chain(fakes.players.into_iter().map(|(desc, _emu)| desc),)
                    .collect();
                let enter_battle_ev = crate::events::battle_enter::BattleEnter {
                    host: hostname.clone(),
                    port: hostport,
                    map: key.map.clone(),
                    mode: key.mode,
                    guid: guid_str.clone(),
                    is_ranked: false,
                    is_custom: false,
                    visibility: Some(key.visibility),
                    auto_heal: key.auto_heal,
                    player_datas,
                    network_config: network_conf.clone(),
                };
                let arc_event = std::sync::Arc::new(enter_battle_ev);
                for player in players.iter() {
                    tokio::spawn(Self::send_events_to_player(arc_event.clone(), player.emitter.clone()));
                }
                log::info!("{} players are entering match {}", players.len(), guid_str);
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

    pub async fn join_queue(&self, map: String, mode: oj_rc_core::data::game_mode::GameMode, visibility: oj_rc_core::data::game_mode::MapVisibility, auto_heal: bool, user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>, event_emitter: polariton_server::events::EventEmitter) {
        if !self.is_enabled {
            event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                code: oj_rc_core::data::error_codes::LobbyReasonCode::NoSuitableLobbyFound as i16,
                text: "Multiplayer is not enabled".to_owned(),
            });
            return;
        }

        self.ensure_autostart_task_running();

        let key = QueueKey {
            map, mode, visibility, auto_heal,
        };
        let lobby_user = user.as_ref().as_ref();
        match lobby_user.player_data(&self.cpu_counter).await {
            Ok(player_data) => {
                let new_player = QueueUser {
                    emitter: event_emitter,
                    player: player_data,
                    user_id: oj_rc_core::persist::user::LobbyUser::user_id(lobby_user),
                    enqueued_at: chrono::Utc::now(),
                    user: user.clone(),
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

                            for users in new_queue_map.values_mut() {
                                users.sort_by_key(|u| u.enqueued_at);
                            }
                            *lock = new_queue_map;
                            if count != 0 {
                                log::info!("Upgraded {} users in queue to new gamemode {}", count, key.display_guid());
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
                                log::info!("Notified {} users in queue of new gamemode {}", count, key.display_guid());
                            }
                        },
                        GamemodeChangeStrategy::Ignore => {
                            log::debug!("Gamemode appears to have changed to {}, ignoring already-queued players", key.display_guid());
                        }
                    }
                }
                let players_len = if let Some(players) = lock.get_mut(&key) {
                    log::info!("User {} entered queue for existing match {}", new_player.user_id, key.display_guid());
                    players.push(new_player);
                    players.len()
                } else {
                    log::info!("User {} entered queue for new match {}", new_player.user_id, key.display_guid());
                    lock.insert(key.clone(), vec![new_player]);
                    1
                };
                let game_ready = players_len >= self.users_per_game;
                let players = if game_ready { lock.remove(&key) } else { None };
                drop(lock);
                if let Some(players) = players {
                    self.enter_match(key, players, lobby_user).await;
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

    pub async fn leave_queue(&self, user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>) {
        let user_id = oj_rc_core::persist::user::LobbyUser::user_id(user.as_ref().as_ref());
        for queue in self.users_in_queue.lock().await.values_mut() {
            if let Some((i, _)) = queue.iter().enumerate().find(|(_, user)| user.user_id == user_id) {
                queue.remove(i);
                log::info!("User {} was removed from a queue", user_id);
                break;
            }
        }
    }
}
