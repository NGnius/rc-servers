use std::{collections::HashMap, hash::Hash};

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
}

impl QueueHandler {
    pub fn new(conf: &oj_rc_core::ConfigImpl, game_host: &str) -> Self {
        let (domain, port_str) = game_host.split_once(':').expect("Invalid redirect address (must be domain:port)");
        Self {
            users_in_queue: tokio::sync::Mutex::new(HashMap::new()),
            users_per_game: oj_rc_core::ConfigProvider::<()>::players_per_game(conf),
            is_enabled: oj_rc_core::ConfigProvider::<()>::is_multiplayer_enabled(conf),
            hostname: domain.to_owned(),
            hostport: port_str.parse().expect("Invalid redirect port"),
            network_conf: crate::data::network::NetworkConfigData::from_conf(oj_rc_core::ConfigProvider::<()>::network_config(conf)),
        }
    }

    async fn enter_match(&self, key: QueueKey, players: Vec<QueueUser>, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync)) {
        use std::hash::Hasher;
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        let guid = oj_rc_core::persist::user::uuid_sanitize(hasher.finish() as i64);
        let guid_str = oj_rc_core::persist::user::i64_as_uuid_str(guid);
        let player_descs = players.iter().map(|x| oj_rc_core::persist::user::PlayerLobbyDescriptor {
            user_id: x.user_id,
            team: x.player.team,
            group: None, // TODO support platoons
        }).collect();
        let game_desc = oj_rc_core::persist::user::GameDescriptor {
            guid: guid_str.clone(),
            map: key.map.clone(),
            mode: key.mode.clone(),
            visibility: key.visibility.clone(),
            auto_heal: key.auto_heal,
            is_ranked: false,
            is_custom: false,
            is_complete: false,
        };
        match user.start_game(game_desc, player_descs).await {
            Ok(_) => {
                let player_datas = players.iter().map(|x| x.player.clone()).collect();
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
        match user.player_data().await {
            Ok(player_data) => {
                let mut new_player = QueueUser {
                    emitter: event_emitter,
                    player: player_data,
                    user_id: user.user_id(),
                };
                let mut lock = self.users_in_queue.lock().await;
                let players_len = if let Some(players) = lock.get_mut(&key) {
                    new_player.player.team = (players.len() % 2) as _; // alternate teams
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
}
