use std::{collections::HashMap, hash::Hash};

use oj_rc_core::persist::user::TeamChooser;

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
    fn short(&self) -> impl std::fmt::Display {
        let mode = match self.mode {
            oj_rc_core::data::game_mode::GameMode::BattleArena => "BA",
            oj_rc_core::data::game_mode::GameMode::SuddenDeath => "Classic",
            oj_rc_core::data::game_mode::GameMode::Pit => "P",
            oj_rc_core::data::game_mode::GameMode::TestMode => "T",
            oj_rc_core::data::game_mode::GameMode::SinglePlayer => "SP",
            oj_rc_core::data::game_mode::GameMode::TeamDeathmatch => "TDM",
            oj_rc_core::data::game_mode::GameMode::Campaign => "C",
        };
        let visibility = match self.visibility {
            oj_rc_core::data::game_mode::MapVisibility::Good => "0",
            oj_rc_core::data::game_mode::MapVisibility::Poor => "o",
            oj_rc_core::data::game_mode::MapVisibility::Bad => ".",
        };
        let auto_heal = if self.auto_heal { "+" } else { "" };
        format!("{}@{}|{}{}", mode, self.map, visibility, auto_heal)
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

struct Queue {
    users: Vec<QueueUser>,
    platoons: HashMap<String, Platoon>,
}

struct Platoon {
    total: u8,
    members: Vec<PlatoonUser>,
}

/// A QueueUser wrapper for platoon members
struct PlatoonUser(QueueUser);

impl PlatoonUser {
    fn from_queue_user(qu: QueueUser) -> Self {
        Self(qu)
    }

    fn clone_for_queue(&self) -> QueueUser {
        QueueUser {
            emitter: self.0.emitter.clone(),
            player: self.0.player.clone(),
            user_id: self.0.user_id,
            enqueued_at: self.0.enqueued_at,
            user: self.0.user.clone(),
        }
    }
}

pub struct PlatoonInfo {
    pub total: u8,
    pub platoon_id: String,
    pub is_leader: bool,
}

struct QueueUser {
    emitter: polariton_server::events::EventEmitter,
    player: oj_rc_core::data::player_data::PlayerData,
    user_id: i32,
    enqueued_at: chrono::DateTime<chrono::Utc>,
    user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
}

struct CustomGameQueue {
    config: oj_rc_core::persist::user::intercom::IntercomLobbyCustomGameConfig,
    users: Vec<CustomGameQueueUser>,
}

struct CustomGameQueueUser {
    public_id: String,
    team: u8,
    queue_user: Option<QueueUser>, // if None, the user is not enqueued
}

pub struct QueueHandler {
    users_in_queue: std::sync::Arc<tokio::sync::Mutex<HashMap<QueueKey, Queue>>>,
    users_in_custom_games_queue: std::sync::Arc<tokio::sync::Mutex<HashMap<String, CustomGameQueue>>>,
    custom_game_for_user: std::sync::Arc<tokio::sync::RwLock<HashMap<String, String>>>,
    users_per_game: usize,
    is_enabled: bool,
    is_parties_enabled: bool,
    hostname: std::sync::Arc<String>,
    hostport: u16,
    network_conf: crate::data::network::NetworkConfigData,
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
    weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    change_strategy: GamemodeChangeStrategy,
    autostart_after: Option<std::time::Duration>,
    autostart_task_started: std::sync::atomic::AtomicBool,
    team_choosers: std::sync::Arc<crate::team_selection::InitedTeamChoosers>,
    wait_times: std::sync::Arc<super::queue_time_tracker::QueueTimeTracker>,
}

impl QueueHandler {
    pub fn new(
        conf: &oj_rc_core::ConfigImpl,
        game_host: &str,
        factory: std::sync::Arc<oj_rc_core::factory::Factory>,
        cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
        weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
        team_choosers: crate::team_selection::InitedTeamChoosers,
    ) -> Self {
        let (name, port) = if let Ok(socket_addr) = game_host.parse::<std::net::SocketAddr>() {
            log::debug!("Parsed game host redirect as raw IP address and port");
            (socket_addr.ip().to_string(), socket_addr.port())
        } else if let Some((name, port)) = game_host.rsplit_once(':') {
            log::debug!("Parsed game host redirect as domain and port");
            (name.to_owned(), port.parse().expect("Invalid redirect port"))
        } else {
            panic!("Invalid multiplayer redirect host address");
        };
        let mp_settings = oj_rc_core::ConfigProvider::<()>::multiplayer_settings(conf);
        let server_conf = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::server_config(conf);
        Self {
            users_in_queue: std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            users_in_custom_games_queue: std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            custom_game_for_user: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            users_per_game: oj_rc_core::ConfigProvider::<()>::players_per_game(conf),
            is_enabled: mp_settings.is_enabled,
            is_parties_enabled: server_conf.allow_parties,
            hostname: std::sync::Arc::new(name),
            hostport: port,
            network_conf: crate::data::network::NetworkConfigData::from_conf(oj_rc_core::ConfigProvider::<()>::network_config(conf)),
            factory,
            cpu_counter,
            weapon_guesser,
            change_strategy: GamemodeChangeStrategy::from_core(server_conf.queue_mode),
            autostart_after: mp_settings.lobby_autostart_after,
            autostart_task_started: std::sync::atomic::AtomicBool::new(false),
            team_choosers: std::sync::Arc::new(team_choosers),
            wait_times: std::sync::Arc::new(super::queue_time_tracker::QueueTimeTracker::new()),
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
        let team_choosers = self.team_choosers.clone();
        let wait_times = self.wait_times.clone();

        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now();
                let mut to_start: Vec<(QueueKey, Queue)> = Vec::new();
                let mut next_deadline: Option<chrono::DateTime<chrono::Utc>> = None;

                {
                    let mut lock = users_in_queue.lock().await;

                    //let mut empty_keys: Vec<QueueKey> = Vec::new();
                    let mut expired_keys: Vec<QueueKey> = Vec::new();

                    for (key, q_entry) in lock.iter() {
                        if q_entry.users.is_empty() { continue; }
                        /*if q_entry.users.is_empty() && q_entry.platoons.is_empty() {
                            empty_keys.push(key.clone());
                            continue;
                        }*/

                        // first user is always oldest within a queue
                        let deadline = q_entry.users[0].enqueued_at + autostart_after;
                        if now >= deadline {
                            expired_keys.push(key.clone());
                        } else {
                            next_deadline = Some(match next_deadline {
                                Some(d) => if deadline < d { deadline } else { d },
                                None => deadline,
                            });
                        }
                    }

                    /*for k in empty_keys {
                        lock.remove(&k);
                    }*/

                    for k in expired_keys {
                        if let Some(q_entry) = lock.remove(&k) {
                            if !q_entry.users.is_empty() {
                                to_start.push((k, q_entry));
                            }
                        }
                    }
                }

                // start expired queues outside the lock
                for (key, q_entry) in to_start {
                    // choose the oldest queued user's LobbyUser handle
                    let starter = match q_entry.users.first() {
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
                        q_entry,
                        starter.as_ref().as_ref(),
                        team_choosers.as_ref(),
                        wait_times.as_ref(),
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

    async fn enter_match(&self, key: QueueKey, q_entry: Queue, user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync)) {
        Self::enter_match_static(
            self.hostname.clone(),
            self.hostport,
            self.network_conf.clone(),
            self.factory.clone(),
            self.cpu_counter.clone(),
            self.weapon_guesser.clone(),
            self.users_per_game,
            key,
            q_entry,
            user,
            &self.team_choosers,
            &self.wait_times,
        ).await
    }

    fn team_selector(choosers: &crate::team_selection::InitedTeamChoosers, mode: oj_rc_core::data::game_mode::GameMode) -> &'_ oj_rc_core::persist::user::StandardTeamChooser {
        match mode {
            oj_rc_core::data::game_mode::GameMode::BattleArena => &choosers.battle_arena,
            oj_rc_core::data::game_mode::GameMode::SuddenDeath => &choosers.elimination,
            oj_rc_core::data::game_mode::GameMode::TeamDeathmatch => &choosers.team_deathmatch,
            oj_rc_core::data::game_mode::GameMode::Pit => &choosers.pit,
            x => {
                log::warn!("No team selector available for multiplayer mode {:?}; using elimination", x);
                &choosers.elimination
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn enter_match_static(
        hostname: std::sync::Arc<String>,
        hostport: u16,
        network_conf: crate::data::network::NetworkConfigData,
        factory: std::sync::Arc<oj_rc_core::factory::Factory>,
        cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
        weapon_guesser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
        users_per_game: usize,
        key: QueueKey,
        mut q_entry: Queue,
        user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync),
        team_choosers: &crate::team_selection::InitedTeamChoosers,
        wait_times: &crate::queue_time_tracker::QueueTimeTracker,
    ) {
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
            overrides: None,
        };
        let team_picker = Self::team_selector(team_choosers, game_desc.mode);
        let mut player_descs = Vec::with_capacity(q_entry.users.len());
        for (i, player) in q_entry.users.iter_mut().enumerate() {
            let mut lobby_desc = oj_rc_core::persist::user::PlayerLobbyDescriptor {
                user_id: player.user_id,
                team: -1,
                group: player.player.group.clone(),
                public_id: player.player.name.clone(),
                display_name: player.player.display_name.clone(),
            };
            let team = team_picker.choose_team(&guid_str, i, &lobby_desc);
            lobby_desc.team = team;
            player.player.team = team;
            player_descs.push(lobby_desc);
        }

        wait_times.update_time_match_starting_now(q_entry.users.iter().map(|x| x.enqueued_at));

        let missing = users_per_game.saturating_sub(q_entry.users.len());

        match user.start_game(game_desc, player_descs, factory.as_ref(), &cpu_counter, &weapon_guesser, team_picker, missing).await {
            Ok(fakes) => {
                let player_datas = q_entry.users.iter().map(|x| x.player.clone())
                    .chain(fakes.players.into_iter().map(|(desc, _emu)| desc))
                    .collect();
                let enter_battle_ev = crate::events::battle_enter::BattleEnter {
                    host: hostname.to_string(),
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
                for player in q_entry.users.iter() {
                    tokio::spawn(Self::send_events_to_player(arc_event.clone(), player.emitter.clone()));
                }
                log::info!("{} players are entering match {}", q_entry.users.len(), guid_str);
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

    async fn enter_custom_match(
        &self,
        _session_id: &str,
        session: &mut CustomGameQueue,
        user: &(dyn oj_rc_core::persist::user::LobbyUser + Send + Sync),
    ) {
        let mode = match session.config.game_mode {
            oj_rc_core::persist::user::intercom::CustomGameMode::BattleArena => oj_rc_core::data::game_mode::GameMode::BattleArena,
            oj_rc_core::persist::user::intercom::CustomGameMode::TeamDeathmatch => oj_rc_core::data::game_mode::GameMode::TeamDeathmatch,
            oj_rc_core::persist::user::intercom::CustomGameMode::Pit => oj_rc_core::data::game_mode::GameMode::Pit,
            oj_rc_core::persist::user::intercom::CustomGameMode::SuddenDeath => oj_rc_core::data::game_mode::GameMode::SuddenDeath,
        };
        let visibility = match session.config.map_visibility {
            oj_rc_core::persist::user::intercom::CustomGameVisibility::Good => oj_rc_core::data::game_mode::MapVisibility::Good,
            oj_rc_core::persist::user::intercom::CustomGameVisibility::Poor => oj_rc_core::data::game_mode::MapVisibility::Poor,
            oj_rc_core::persist::user::intercom::CustomGameVisibility::Bad => oj_rc_core::data::game_mode::MapVisibility::Bad,
        };
        let key = QueueKey {
            map: session.config.map.clone(),
            mode,
            visibility,
            auto_heal: session.config.health_regen,
        };
        let guid_str = key.unique_guid();
        let game_desc = oj_rc_core::persist::user::GameDescriptor {
            guid: guid_str.clone(),
            map: session.config.map.clone(),
            mode,
            visibility,
            auto_heal: session.config.health_regen,
            is_ranked: false,
            is_custom: true,
            is_complete: false,
            overrides: Some(session.config.as_core()),
        };
        let player_descs = session.users.iter()
            .map(|user| {
                let q_user = user.queue_user.as_ref().unwrap();
                oj_rc_core::persist::user::PlayerLobbyDescriptor {
                    user_id: q_user.user_id,
                    team: user.team as i32,
                    group: None,
                    public_id: q_user.player.name.clone(),
                    display_name: q_user.player.display_name.clone(),
                }
            })
            .collect();
        match user.start_custom_game(game_desc, player_descs).await {
            Ok(_) => {
                let player_datas = session.users.iter()
                    .map(|x| x.queue_user.as_ref().unwrap().player.clone())
                    .collect();
                let enter_battle_ev = crate::events::battle_enter::BattleEnter {
                    host: self.hostname.to_string(),
                    port: self.hostport,
                    map: session.config.map.clone(),
                    mode,
                    guid: guid_str.clone(),
                    is_ranked: false,
                    is_custom: true,
                    visibility: Some(key.visibility),
                    auto_heal: key.auto_heal,
                    player_datas,
                    network_config: self.network_conf.clone(),
                };
                let arc_event = std::sync::Arc::new(enter_battle_ev);
                for user in session.users.iter_mut() {
                    let player = user.queue_user.take().unwrap();
                    tokio::spawn(Self::send_events_to_player(arc_event.clone(), player.emitter.clone()));
                }
                log::info!("{} players are entering custom match {}; {}", session.users.len(), guid_str, key.short());
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
        const WAIT_BEFORE_ENTER: std::time::Duration = std::time::Duration::from_millis(10);
        if sender.emit(crate::events::battle_found::BattleFound) {
            tokio::time::sleep(WAIT_BEFORE_ENTER).await;
            sender.emit(enter_event.as_ref());
        }
    }

    pub async fn join_custom_queue(&self, user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>, event_emitter: polariton_server::events::EventEmitter) {
        if !self.is_enabled {
            event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                code: oj_rc_core::data::error_codes::LobbyReasonCode::NoSuitableLobbyFound as i16,
                text: "Multiplayer is not enabled".to_owned(),
            });
            return;
        }
        let public_id = user.public_id();
        if let Some(session_id) = self.custom_game_for_user.read().await.get(public_id) {
            let mut lock = self.users_in_custom_games_queue.lock().await;
            let session = lock.get_mut(session_id).unwrap();
            let lobby_user = user.as_ref().as_ref();
            match lobby_user.player_data(&self.cpu_counter).await {
                Ok(mut player_data) => {
                    let target_queuer = session.users.iter_mut()
                        .find(|user| user.public_id == public_id)
                        .unwrap();
                    player_data.team = target_queuer.team as i32;
                    let new_player = QueueUser {
                        emitter: event_emitter,
                        player: player_data,
                        user_id: oj_rc_core::persist::user::LobbyUser::user_id(lobby_user),
                        enqueued_at: chrono::Utc::now(),
                        user: user.clone(),
                    };
                    target_queuer.queue_user = Some(new_player);
                },
                Err(e) => {
                    event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                        code: e.error_code(),
                        text: e.error_msg().map(|x| x.to_owned()).unwrap_or_else(|| "Unknown queue join error".to_owned()),
                    });
                    return;
                }
            }
            let is_all_enqueued = session.users.iter().all(|user| user.queue_user.is_some());
            if is_all_enqueued {
                self.enter_custom_match(session_id, session, lobby_user).await;
            }
        } else {
            log::debug!("Rejecting join queue for unknown custom game for user {}", public_id);
            event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                code: oj_rc_core::data::error_codes::LobbyReasonCode::NoSuitableLobbyFound as i16,
                text: "User is not in a custom game".to_owned(),
            });
        }
    }

    pub async fn remove_custom_queue(&self, session_id: &str) -> bool {
        log::debug!("Disbanding custom game {}", session_id);
        self.custom_game_for_user.write().await.retain(|_key, val| val != session_id);
        let mut lock = self.users_in_custom_games_queue.lock().await;
        lock.remove(session_id).is_some()
    }

    pub async fn update_custom_queue(&self, session_id: &str, members: impl std::iter::Iterator<Item = (String, u8)>, config: oj_rc_core::persist::user::intercom::IntercomLobbyCustomGameConfig) -> bool {
        let mut lock = self.users_in_custom_games_queue.lock().await;
        let members: std::collections::HashMap<_, _> = members.collect();
        let mut user_map_lock = self.custom_game_for_user.write().await;
        if let Some(session) = lock.get_mut(session_id) {
            log::debug!("Updating existing custom game {}", session_id);
            let member_ids: std::collections::HashSet<_> = members.keys().collect();
            // remove users who are no longer part of the custom game
            user_map_lock.retain(|key, val| (members.contains_key(key) && val == session_id) || (!members.contains_key(key) && val != session_id));
            session.users.retain(|user| member_ids.contains(&user.public_id));
            // update team assignments
            session.users.iter_mut()
                .for_each(|user| {
                    let new_team = *members.get(&user.public_id).unwrap();
                    user.team = new_team;
                    if let Some(q_user) = &mut user.queue_user {
                        q_user.player.team = new_team as i32;
                    }
                });
            // collect users which are still members
            let existing_ids: std::collections::HashSet<_> = session.users.iter()
                .map(|mem| mem.public_id.clone())
                .collect();
            // add new members
            for (id, team) in members.iter() {
                if !existing_ids.contains(id) {
                    session.users.push(CustomGameQueueUser {
                        public_id: id.to_owned(),
                        team: *team,
                        queue_user: None,
                    });
                    user_map_lock.insert(id.to_owned(), session_id.to_owned());
                }
            }
            // update config overrides
            session.config = config;
            false
        } else {
            log::debug!("Creating new custom game {}", session_id);
            lock.insert(session_id.to_owned(), CustomGameQueue {
                config,
                users: members.iter().map(|(mem_id, team)| {
                    user_map_lock.insert(mem_id.to_owned(), session_id.to_owned());
                    CustomGameQueueUser {
                        public_id: mem_id.to_owned(),
                        team: *team,
                        queue_user: None,
                    }
                })
                .collect()
            });
            true
        }
    }

    fn join_or_create_platoon(&self, new_player: QueueUser, platoon: PlatoonInfo, q_entry: &mut Queue) {
        let new_player = PlatoonUser::from_queue_user(new_player);
        log::debug!("Existing platoon count {}", q_entry.platoons.len());
        if let Some(p_entry) = q_entry.platoons.get_mut(&platoon.platoon_id) {
            // update
            log::debug!("Adding user {} to existing platoon {} ({}/{})", new_player.0.user_id, platoon.platoon_id, p_entry.members.len() + 1, p_entry.total);
            if platoon.is_leader {
                p_entry.total = platoon.total;
            }
            p_entry.members.push(new_player);
            let wants_usize = p_entry.total as usize;
            if wants_usize == p_entry.members.len() {
                log::info!("All members of platoon {} are in queue, actually adding them to queue", platoon.platoon_id);
                for member in p_entry.members.iter() {
                    q_entry.users.push(member.clone_for_queue());
                }
                q_entry.users.sort_by_key(|u| u.enqueued_at);
            }
        } else {
            // create
            log::debug!("Adding user {} to new platoon {} (1/{})", new_player.0.user_id, platoon.platoon_id, platoon.total);
            q_entry.platoons.insert(platoon.platoon_id.clone(), Platoon {
                total: if platoon.is_leader { platoon.total } else { u8::MAX },
                members: vec![new_player],
            });
        }
    }

    fn platoon_leave_queue(&self, user_id: i32, platoon_id: String, q_entry: &mut Queue) {
        let needs_remove = if let Some(platoon) = q_entry.platoons.get_mut(&platoon_id) {
            let wants_usize = platoon.total as usize;
            if wants_usize == platoon.members.len() {
                // other platoon members are in queue, they also need to be removed
                let some_group = Some(platoon_id.clone());
                q_entry.users.retain(|q_user| q_user.player.group != some_group);
                log::info!("Removed platoon {} from actual queue because user {} left queue", platoon_id, user_id);
            }
            platoon.members.retain(|p_mem| p_mem.0.user_id != user_id);
            platoon.members.is_empty()
        } else {
            log::error!("Failed to find platoon {} to remove user {}: the lobby is in a bad state", platoon_id, user_id);
            false
        };
        if needs_remove {
            q_entry.platoons.remove(&platoon_id);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn join_queue(&self, map: String, mode: oj_rc_core::data::game_mode::GameMode, visibility: oj_rc_core::data::game_mode::MapVisibility, auto_heal: bool, user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>, event_emitter: polariton_server::events::EventEmitter, platoon: Option<PlatoonInfo>) {
        if !self.is_enabled {
            event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                code: oj_rc_core::data::error_codes::LobbyReasonCode::NoSuitableLobbyFound as i16,
                text: "Multiplayer is not enabled".to_owned(),
            });
            return;
        }
        if let Some(platoon_info) = &platoon {
            if platoon_info.total != 1 && !self.is_parties_enabled {
                event_emitter.emit(crate::events::enqueue_error::QueueJoinError {
                    code: oj_rc_core::data::error_codes::LobbyReasonCode::PartyNotAllowed as i16,
                    text: "Platoons are not enabled".to_owned(),
                });
                return;
            }
        }

        self.ensure_autostart_task_running();

        let key = QueueKey {
            map, mode, visibility, auto_heal,
        };
        let lobby_user = user.as_ref().as_ref();
        match lobby_user.player_data(&self.cpu_counter).await {
            Ok(mut player_data) => {
                if let Some(platoon) = &platoon {
                    player_data.group = Some(platoon.platoon_id.clone());
                }
                let new_player = QueueUser {
                    emitter: event_emitter,
                    player: player_data,
                    user_id: oj_rc_core::persist::user::LobbyUser::user_id(lobby_user),
                    enqueued_at: chrono::Utc::now(),
                    user: user.clone(),
                };

                let mut lock = self.users_in_queue.lock().await;
                for q_key in lock.keys() {
                    log::debug!("Existing queue for {} (matches joiner? {})", q_key.short(), q_key.short().to_string() == key.short().to_string());
                }
                // handle game event change
                if !lock.contains_key(&key) && !lock.is_empty() {
                    log::debug!("Game event change detected");
                    match self.change_strategy {
                        GamemodeChangeStrategy::Upgrade => {
                            let mut new_queue_map = std::collections::HashMap::<QueueKey, Queue>::with_capacity(lock.len());
                            let mut count = 0;
                            for (_key, mut q_entry) in lock.drain() {
                                count += q_entry.users.len();
                                if let Some(values) = new_queue_map.get_mut(&key) {
                                    values.users.append(&mut q_entry.users);
                                    values.platoons.extend(q_entry.platoons);
                                } else {
                                    new_queue_map.insert(key.clone(), q_entry);
                                }
                            }

                            for q_entry in new_queue_map.values_mut() {
                                q_entry.users.sort_by_key(|u| u.enqueued_at);
                            }
                            *lock = new_queue_map;
                            if count != 0 {
                                log::info!("Upgraded {} users in queue to new gamemode {}", count, key.short());
                            }
                        },
                        GamemodeChangeStrategy::Notify => {
                            let mut seen = std::collections::HashSet::new();
                            for (_key, q_entry) in lock.drain() {
                                for player in q_entry.users {
                                    if seen.contains(&player.user_id) { continue; }
                                    log::debug!("Notifying user {} of event expiry", player.user_id);
                                    player.emitter.emit(crate::events::enqueue_error::QueueJoinError {
                                        code: oj_rc_core::data::error_codes::LobbyReasonCode::EventSystemExpired as i16,
                                        text: "Please requeue".to_owned(),
                                    });
                                    seen.insert(player.user_id);
                                }
                                for platoon in q_entry.platoons.into_values() {
                                    for player in platoon.members {
                                        if seen.contains(&player.0.user_id) { continue; }
                                        log::debug!("Notifying user {} of event expiry", player.0.user_id);
                                        player.0.emitter.emit(crate::events::enqueue_error::QueueJoinError {
                                            code: oj_rc_core::data::error_codes::LobbyReasonCode::EventSystemExpired as i16,
                                            text: "Please requeue".to_owned(),
                                        });
                                        seen.insert(player.0.user_id);
                                    }
                                }
                            }
                            if !seen.is_empty() {
                                log::info!("Notified {} users in queue of new gamemode {}", seen.len(), key.short());
                            }
                        },
                        GamemodeChangeStrategy::Ignore => {
                            log::debug!("Gamemode appears to have changed to {}, ignoring already-queued players", key.short());
                        }
                    }
                }
                let players_len = if let Some(q_entry) = lock.get_mut(&key) {
                    if let Some(platoon) = platoon {
                        log::info!("User {} platooned {} queue for existing match {}", new_player.user_id, platoon.platoon_id, key.short());
                        self.join_or_create_platoon(new_player, platoon, q_entry);
                    } else {
                        log::info!("User {} entered queue for existing match {}", new_player.user_id, key.short());
                        q_entry.users.push(new_player);
                    }
                    q_entry.users.len()
                } else {
                    if let Some(platoon) = platoon {
                        log::info!("User {} platooned {} queue for new match {}", new_player.user_id, platoon.platoon_id, key.short());
                        let mut q_entry = Queue {
                            users: Vec::default(),
                            platoons: HashMap::default(),
                        };
                        self.join_or_create_platoon(new_player, platoon, &mut q_entry);
                        lock.insert(key.clone(), q_entry);
                        0
                    } else {
                        log::info!("User {} entered queue for new match {}", new_player.user_id, key.short());
                        lock.insert(key.clone(), Queue {
                            users: vec![new_player],
                            platoons: HashMap::default(),
                        });
                        1
                    }
                };
                for q_key in lock.keys() {
                    log::debug!("Now queue for {} (matches joiner? {})", q_key.short(), q_key.short().to_string() == key.short().to_string());
                }
                let game_ready = players_len >= self.users_per_game;
                let players = if game_ready { lock.remove(&key) } else { None };
                drop(lock);
                if let Some(players) = players {
                    self.enter_match(key, players, lobby_user).await;
                }
                log::debug!("join_queue complete success");
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
        let public_id = user.public_id();
        let user_id = oj_rc_core::persist::user::LobbyUser::user_id(user.as_ref().as_ref());
        if let Some(session_id) = self.custom_game_for_user.read().await.get(public_id) {
            // player is in custom game session
            let mut lock = self.users_in_custom_games_queue.lock().await;
            let session = lock.get_mut(session_id).unwrap();
            let target = session.users.iter_mut().find(|user| user.public_id == public_id).unwrap();
            target.queue_user = None;
            log::info!("User {} was removed from custom game {} queue", user_id, session_id);
        } else {
            // fallback to regular multiplayer
            let mut to_remove = Vec::new();
            let mut q_lock = self.users_in_queue.lock().await;
            for (queue_key, queue) in q_lock.iter_mut() {
                if let Some((i, _)) = queue.users.iter().enumerate().find(|(_, user)| user.user_id == user_id) {
                    let q_user = queue.users.remove(i);
                    if let Some(platoon_id) = q_user.player.group {
                        self.platoon_leave_queue(user_id, platoon_id, queue);
                    }
                    if queue.users.is_empty() && queue.platoons.is_empty() {
                        to_remove.push(queue_key.to_owned());
                    }
                    log::info!("User {} was removed from queue {}", user_id, queue_key.short());
                    break;
                }
            }
            for key in to_remove {
                q_lock.remove(&key);
            }
        }
    }

    pub fn wait_time_s(&self) -> i32 {
        self.wait_times.get_average()
    }
}
