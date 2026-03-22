pub struct GameMatches {
    matches: std::collections::HashMap<String, tokio::sync::mpsc::Sender<super::GameMessage>>,
    routing: std::collections::HashMap<i32, String>, // user id to game guid
    mode_configs: oj_rc_core::data::game_mode::GameModeConfigs,
    map_configs: std::collections::HashMap<String, oj_rc_core::persist::config::MapConfig>,
    //fake_players: Vec<oj_rc_core::persist::config::FakePlayer>,
    cube_parsers: std::sync::Arc<oj_rc_core::cubes::CubeParsers>,
    ba_sorted_crystals: std::sync::Arc<tokio::sync::RwLock<std::sync::Arc<Vec<oj_rc_core::cubes::CubeLocationInfo>>>>, // cached after first calculation
    ba_settings: std::sync::Arc<oj_rc_core::persist::config::BattleArenaResolver>,
    pit_settings: std::sync::Arc<oj_rc_core::persist::config::PitSettings>,
    tdm_settings: std::sync::Arc<oj_rc_core::persist::config::TeamDeathMatchSettings>,
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    mp_settings: std::sync::Arc<oj_rc_core::persist::config::MultiplayerSettings>,
    is_crystal_regen_running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl GameMatches {
    pub fn new(conf: &oj_rc_core::persist::config::ConfigImpl, cube_parsers: std::sync::Arc<oj_rc_core::cubes::CubeParsers>, factory: std::sync::Arc<oj_rc_core::factory::Factory>) -> Self {
        let ba_settings = std::sync::Arc::new(<oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::ba_settings(conf));
        let ba_sorted_crystals = std::sync::Arc::new(Self::maybe_init_ba_sorted_crystals(&ba_settings, &cube_parsers));
        Self {
            matches: std::collections::HashMap::new(),
            routing: std::collections::HashMap::new(),
            mode_configs: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::gamemodes(conf),
            map_configs: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::maps(conf)
                .into_iter()
                .map(|(map, conf)| (oj_rc_core::data::game_mode::GameMap::from_persist(map).as_str().to_owned(), conf))
                .collect(),
            //fake_players: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::fake_players(conf),
            cube_parsers,
            ba_sorted_crystals,
            ba_settings,
            pit_settings: std::sync::Arc::new(<oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::pit_settings(conf)),
            tdm_settings: std::sync::Arc::new(<oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::tdm_settings(conf)),
            factory,
            mp_settings: std::sync::Arc::new(<oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::multiplayer_settings(conf)),
            is_crystal_regen_running: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    fn maybe_init_ba_sorted_crystals(ba_conf: &oj_rc_core::persist::config::BattleArenaResolver, cube_parsers: &oj_rc_core::cubes::CubeParsers) -> tokio::sync::RwLock<std::sync::Arc<Vec<oj_rc_core::cubes::CubeLocationInfo>>> {
        if let Some(base_machine_map) = ba_conf.resolve_base_machine_immediate_early() {
            let gen_start = chrono::Utc::now();
            let gen_params = ba_conf.crystal_sort_params();
            let crystals = Self::generate_ordered_crystal_list(&gen_params, cube_parsers, &base_machine_map);
            let gen_end = chrono::Utc::now();
            let delta = gen_end.signed_duration_since(gen_start);
            log::info!("Early base crystal list initialization took {}ms for {} crystals", delta.num_milliseconds(), crystals.len());
            tokio::sync::RwLock::new(std::sync::Arc::new(crystals))
        } else {
            log::warn!("First Batte Arena match will take longer to initialization due to non-raw base machine");
            tokio::sync::RwLock::new(std::sync::Arc::new(Vec::default()))
        }
    }

    fn generate_ordered_crystal_list(
        gen_params: &oj_rc_core::persist::config::BattleArenaCrystalParams,
        cube_parsers: &oj_rc_core::cubes::CubeParsers,
        vehicle_data: &[u8],
    ) -> Vec<oj_rc_core::cubes::CubeLocationInfo> {
        cube_parsers.locations_of()
            .locations_of_reactor_sort_custom(
                &mut std::io::Cursor::new(vehicle_data),
                gen_params.max_iterations,
                gen_params.max_random_iterations,
            )
    }

    pub fn spawn(self) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(super::CHANNEL_BOUND);
        tokio::spawn(self.run(rx));
        tx
    }

    fn build_player_emulator(&self, emu: oj_rc_core::persist::config::ClientEmulator, player: &oj_rc_core::persist::user::PlayerDescriptor) -> Box<dyn super::fake::FakeUser> {
        let owned_player = player.to_owned();
        match emu {
            oj_rc_core::persist::config::ClientEmulator::Experiment => Box::new(super::fake::ExperimentalPlayer::new(owned_player)),
            oj_rc_core::persist::config::ClientEmulator::ClientAI => Box::new(super::fake::ClientAIPlayer::new(owned_player)),
        }
    }

    fn build_fake_players(&self, players: &[oj_rc_core::persist::user::PlayerDescriptor]) -> std::collections::HashMap<u8, Box<dyn super::fake::FakeUser>> {
        let mut fakes = std::collections::HashMap::new();
        for player in players.iter() {
            //if player.user_id.is_some() { continue; }
            if let Some(emu_mode) = player.mode {
                let fake = self.build_player_emulator(emu_mode, player);
                fakes.insert(player.player_id, fake);
            }
        }
        fakes
    }

    async fn start_new_match_engine(&self, user: &(dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static), guid: &str) -> Result<tokio::sync::mpsc::Sender<super::GameMessage>, oj_rc_core::persist::user::MultiplayerError> {
        let game_info = user.game_info(guid).await?
        .ok_or_else(|| oj_rc_core::persist::user::MultiplayerError {
            code: oj_rc_core::persist::user::MultiplayerErrorCode::CustomString,
            message: format!("Failed to find game {}", guid),
        })?;
        let map_config = self.map_configs.get(&game_info.map)
            .map(|x| x.to_owned())
            .unwrap_or_else(|| {
                log::warn!("No configuration found for map {}, game {} may not work correctly", game_info.map, guid);
                oj_rc_core::persist::config::MapConfig {
                    spawns: std::collections::HashMap::default(),
                    pit_spawns: Vec::default(),
                    bases: std::collections::HashMap::default(),
                    capture_points: Vec::default(),
                    equalizer: oj_rc_core::persist::config::Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    }
                }
            });
        let players = user.game_players(guid).await?;
        if players.is_empty() {
            log::warn!("No players found for game {}, loading may not work correctly", guid);
        } else {
            log::info!("There are {} ({} real) players for game {}", players.len(), players.iter().filter(|x| x.user_id.is_some()).count(), guid);
        }
        let fakes = self.build_fake_players(&players);
        let fakes_handler = super::fake::Handler::start(fakes, players.clone()).await;
        match game_info.mode {
            oj_rc_core::data::game_mode::GameMode::SuddenDeath => {
                let super_conf = super::engine::SuperConfig {
                    descriptor: game_info,
                    map: map_config,
                    game_mode: &self.mode_configs.elimination,
                    players,
                };
                let inner = super::modes::EliminationLogic::new(
                    &self.mode_configs.elimination,
                    &super_conf.map,
                    &super_conf.players,
                );
                let engine = super::GenericGamemodeEngine::new(
                    super_conf,
                    inner,
                    fakes_handler,
                    self.mp_settings.clone(),
                );
                Ok(engine.spawn())
            },
            oj_rc_core::data::game_mode::GameMode::BattleArena => {
                log::warn!("Game {}: Battle Arena is experimental", guid);
                let resolved_ba_conf = self.ba_settings.resolve(
                    user,
                    self.factory.as_ref(),
                    &self.cube_parsers.weapon_order(),
                    &self.cube_parsers.cpu_counter(),
                ).await.map_err(|e| oj_rc_core::persist::user::MultiplayerError {
                     code: oj_rc_core::persist::user::MultiplayerErrorCode::CustomString,
                     message: e.error_msg().map(|x| x.to_owned()).unwrap_or_else(|| "Failed to resolve special settings for Battle Arena".to_owned()),
                })?;
                let crystals = if self.ba_sorted_crystals.read().await.is_empty() {
                    let crystals = std::sync::Arc::new(
                        // NOTE: this uses default (lower) crystal sort params to prevent sorting from taking too long
                        // if this takes too long, players can be disconnected from the multiplayer due to client timeout
                        self.cube_parsers.locations_of()
                            .locations_of_reactor_sort(&mut std::io::Cursor::new(&resolved_ba_conf.base_machine_map))
                    );
                    *self.ba_sorted_crystals.write().await = crystals.clone();
                    crystals
                } else {
                    self.ba_sorted_crystals.read().await.clone()
                };
                let super_conf = super::engine::SuperConfig {
                    descriptor: game_info,
                    map: map_config,
                    game_mode: &self.mode_configs.battle_arena,
                    players,
                };
                let inner = super::modes::BattleArenaLogic::new(
                    &super_conf,
                    resolved_ba_conf,
                    crystals,
                );
                let engine = super::GenericGamemodeEngine::new(
                    super_conf,
                    inner,
                    fakes_handler,
                    self.mp_settings.clone(),
                );
                Ok(engine.spawn())
            },
            oj_rc_core::data::game_mode::GameMode::Pit => {
                let super_conf = super::engine::SuperConfig {
                    descriptor: game_info,
                    map: map_config,
                    game_mode: &self.mode_configs.the_pit,
                    players,
                };
                let inner = super::modes::PitLogic::new(
                    &self.mode_configs.the_pit,
                    &super_conf.map,
                    &super_conf.players,
                    self.pit_settings.clone(),
                );
                let engine = super::GenericGamemodeEngine::new(
                    super_conf,
                    inner,
                    fakes_handler,
                    self.mp_settings.clone(),
                );
                Ok(engine.spawn())
            },
            oj_rc_core::data::game_mode::GameMode::TeamDeathmatch => {
                let super_conf = super::engine::SuperConfig {
                    descriptor: game_info,
                    map: map_config,
                    game_mode: &self.mode_configs.team_deathmatch,
                    players,
                };
                let inner = super::modes::TeamDeathMatchLogic::new(
                    &self.mode_configs.team_deathmatch,
                    &super_conf.map,
                    &super_conf.players,
                    self.tdm_settings.clone(),
                );
                let engine = super::GenericGamemodeEngine::new(
                    super_conf,
                    inner,
                    fakes_handler,
                    self.mp_settings.clone(),
                );
                Ok(engine.spawn())
            },
            mode => {
                // TODO support more gamemodes
                Err(oj_rc_core::persist::user::MultiplayerError {
                    code: oj_rc_core::persist::user::MultiplayerErrorCode::CustomString,
                    message: format!("Game mode {:?} is not supported in multiplayer", mode),
                })
            }
        }

    }

    // create a new match
    async fn create_new_game(&mut self,
        user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
        game_guid: String,
        connection: std::sync::Arc<literustlib_server::Connection<crate::PacketData>>,
        response: tokio::sync::oneshot::Sender<Option<super::messages::ErrorMessage>>,
        sender: std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>,
    ) {
        log::info!("Creating new game {}", game_guid);
        let tx = match self.start_new_match_engine(user.as_ref().as_ref(), &game_guid).await {
            Ok(tx) => tx,
            Err(e) => {
                if response.send(Some(crate::matches::messages::ErrorMessage {
                    message: e.message.clone(),
                    inner: Some(Box::new(e)),
                })).is_err() {
                    log::error!("Failed to send NewConnection failure back to event handler");
                }
                return;
            }
        };
        self.matches.insert(game_guid.clone(), tx.clone());
        self.routing.insert(user.user_id(), game_guid.clone());
        if !self.is_crystal_regen_running.swap(true, std::sync::atomic::Ordering::SeqCst) {
            tokio::task::spawn(Self::regenerate_crystal_order_task(
                self.ba_sorted_crystals.clone(),
                self.cube_parsers.clone(),
                self.factory.clone(),
                self.ba_settings.clone(),
                user.clone(),
                self.is_crystal_regen_running.clone(),
            ));
        }
        if tx.send(super::GameMessage::NewConnection { user, game_guid, connection, response, sender }).await.is_err() {
            log::error!("Failed to send NewConnection game message to new match");
        }
    }

    /*fn do_full_cleanup(&mut self) {
        let mut games_to_remove = std::collections::HashSet::new();
        for (game_guid, tx) in self.matches.iter() {
            if tx.is_closed() {
                games_to_remove.insert(game_guid.to_owned());
            }
        }
        self.matches.retain(|key, _| !games_to_remove.contains(key));
        self.routing.retain(|_, val| !games_to_remove.contains(val));
    }*/

    fn do_game_cleanup(&mut self, game_guid: &String) {
        self.routing.retain(|_, game_guid2| game_guid2 != game_guid);
        self.matches.remove(game_guid);
    }

    async fn run(mut self, mut rx: tokio::sync::mpsc::Receiver<super::GameMessage>) {
        log::info!("Match message router has started");
        while !rx.is_closed() {
            if let Some(msg) = rx.recv().await {
                log::trace!("Match message router got a message");
                match msg {
                    super::GameMessage::NewConnection { user, game_guid, connection, response, sender } => {
                        if let Some(tx) = self.matches.get(&game_guid) {
                            if tx.is_closed() {
                                self.do_game_cleanup(&game_guid);
                                self.create_new_game(user.clone(), game_guid, connection, response, sender).await;
                            } else {
                                self.routing.insert(user.user_id(), game_guid.clone());
                                if tx.send(super::GameMessage::NewConnection { user: user.clone(), game_guid, connection, response, sender }).await.is_err() {
                                    log::error!("Failed to send NewConnection game message to existing match");
                                }
                            }
                        } else {
                            self.create_new_game(user.clone(), game_guid, connection, response, sender).await;
                        }
                        crate::update_status(user.as_ref().as_ref(), self.routing.len() as u64).await;
                    },
                    msg => {
                        let user_id = msg.user_id();
                        let mut to_clean = None;
                        if let Some(guid) = self.routing.get(&user_id) {
                            if let Some(tx) = self.matches.get(guid) {
                                if tx.is_closed() {
                                    to_clean = Some(guid.to_owned());
                                } else if tx.send(msg).await.is_err() {
                                    log::error!("Failed to route game message from user {} to match {}", user_id, guid);
                                }
                            } else {
                                self.routing.remove(&user_id);
                            }
                        } else {
                            log::warn!("Got unroutable user {}", user_id);
                        }
                        if let Some(game_guid) = to_clean {
                            self.do_game_cleanup(&game_guid);
                        }
                    }
                }
            }
        }
        log::warn!("Match message router has completed");
    }

    async fn regenerate_crystal_order_task(
        crystal_order: std::sync::Arc<tokio::sync::RwLock<std::sync::Arc<Vec<oj_rc_core::cubes::CubeLocationInfo>>>>,
        parsers: std::sync::Arc<oj_rc_core::cubes::CubeParsers>,
        factory: std::sync::Arc<oj_rc_core::factory::Factory>,
        ba_resolver: std::sync::Arc<oj_rc_core::persist::config::BattleArenaResolver>,
        user: std::sync::Arc<Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>>,
        tracker: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) {
        //tracker.store(true, std::sync::atomic::Ordering::SeqCst);
        let ba_resolved_result = ba_resolver.resolve(
            user.as_ref().as_ref(),
            factory.as_ref(),
            parsers.weapon_order().as_ref(),
            parsers.cpu_counter().as_ref(),
        ).await;
        match ba_resolved_result {
            Ok(ba_resolved) => {
                log::info!("Regenerate crystal order task started successfully, task sleeping for now");
                // wait a long while to prevent this from interfering with the match's startup
                tokio::time::sleep(std::time::Duration::from_secs(5 * 60)).await; // 5 minutes
                let gen_start = chrono::Utc::now();
                let new_order_result = tokio::task::spawn_blocking(move || {
                    Self::generate_ordered_crystal_list(
                        &ba_resolver.crystal_sort_params(),
                        parsers.as_ref(),
                        &ba_resolved.base_machine_map,
                    )
                }).await;
                match new_order_result {
                    Ok(new_order) => {
                        let gen_end = chrono::Utc::now();
                        let delta = gen_end.signed_duration_since(gen_start);
                        log::info!("Base crystal order re-gen took {}ms for {} crystals", delta.num_milliseconds(), new_order.len());
                        *crystal_order.write().await = std::sync::Arc::new(new_order);
                    },
                    Err(e) => {
                        log::error!("Failed to regenerate crystal order: {}", e);
                    }
                }

            },
            Err(e) => {
                if let Some(e_msg) = e.error_msg() {
                    log::error!("Failed to regenerate crystal order: {} ({})", e_msg, e.error_code());
                } else {
                    log::error!("Failed to regenerate crystal order ({})", e.error_code());
                }
            }
        }
        tracker.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
