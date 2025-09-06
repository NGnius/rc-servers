pub struct GameMatches {
    matches: std::collections::HashMap<String, tokio::sync::mpsc::Sender<super::GameMessage>>,
    routing: std::collections::HashMap<i32, String>, // user id to game guid
    mode_configs: oj_rc_core::data::game_mode::GameModeConfigs,
    map_configs: std::collections::HashMap<String, oj_rc_core::persist::config::MapConfig>,
    fake_players: Vec<oj_rc_core::persist::config::FakePlayer>,
    cube_parsers: std::sync::Arc<oj_rc_core::cubes::CubeParsers>,
    ba_settings: std::sync::Arc<oj_rc_core::persist::config::BattleArenaResolver>,
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
}

impl GameMatches {
    pub fn new(conf: &oj_rc_core::persist::config::ConfigImpl, cube_parsers: std::sync::Arc<oj_rc_core::cubes::CubeParsers>, factory: std::sync::Arc<oj_rc_core::factory::Factory>) -> Self {
        Self {
            matches: std::collections::HashMap::new(),
            routing: std::collections::HashMap::new(),
            mode_configs: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::gamemodes(conf),
            map_configs: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::maps(conf)
                .into_iter()
                .map(|(map, conf)| (oj_rc_core::data::game_mode::GameMap::from_persist(map).as_str().to_owned(), conf))
                .collect(),
            fake_players: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::fake_players(conf),
            cube_parsers,
            ba_settings: std::sync::Arc::new(<oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::ba_settings(conf)),
            factory,
        }
    }

    pub fn spawn(self) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(super::CHANNEL_BOUND);
        tokio::spawn(self.run(rx));
        tx
    }

    fn build_player_emulator(&self, emu: oj_rc_core::persist::config::ClientEmulator) -> Box<dyn super::fake::FakeUser> {
        match emu {
            oj_rc_core::persist::config::ClientEmulator::Experiment => Box::new(super::fake::ExperimentalPlayer::new()),
        }
    }

    fn build_fake_players(&self, players: &[oj_rc_core::persist::user::PlayerDescriptor]) -> std::collections::HashMap<u8, Box<dyn super::fake::FakeUser>> {
        let mut fake_player_i = 0;
        let mut fakes = std::collections::HashMap::with_capacity(self.fake_players.len());
        for player in players.iter() {
            if fake_player_i >= self.fake_players.len() { break; }
            if player.user_id.is_none() {
                let fake = self.build_player_emulator(self.fake_players[fake_player_i].implementation);
                fakes.insert(player.player_id, fake);
                fake_player_i += 1;
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
                    bases: std::collections::HashMap::default(),
                    capture_points: Vec::default(),
                }
            });
        let players = user.game_players(guid).await?;
        if players.is_empty() {
            log::warn!("No players found for game {}, loading may not work correctly", guid);
        }
        let fakes = self.build_fake_players(&players);
        let fakes_handler = super::fake::Handler::start(fakes, players.clone()).await;
        match game_info.mode {
            oj_rc_core::data::game_mode::GameMode::SuddenDeath => {
                let inner = super::modes::EliminationLogic::new(&self.mode_configs.elimination, &map_config);
                let engine = super::GenericGamemodeEngine::new(
                    game_info,
                    map_config,
                    &self.mode_configs.elimination,
                    players,
                    inner,
                    fakes_handler,
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
                let inner = super::modes::BattleArenaLogic::new(&self.mode_configs.battle_arena, &map_config, &self.cube_parsers, resolved_ba_conf);
                let engine = super::GenericGamemodeEngine::new(
                    game_info,
                    map_config,
                    &self.mode_configs.battle_arena,
                    players,
                    inner,
                    fakes_handler,
                );
                Ok(engine.spawn())
            }
            mode => {
                // TODO support mode gamemodes
                Err(oj_rc_core::persist::user::MultiplayerError {
                    code: oj_rc_core::persist::user::MultiplayerErrorCode::CustomString,
                    message: format!("Game mode {:?} is not supported (yet)", mode),
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
                log::debug!("Match message router got a message");
                match msg {
                    super::GameMessage::NewConnection { user, game_guid, connection, response, sender } => {
                        if let Some(tx) = self.matches.get(&game_guid) {
                            if tx.is_closed() {
                                self.do_game_cleanup(&game_guid);
                                self.create_new_game(user, game_guid, connection, response, sender).await;
                            } else {
                                self.routing.insert(user.user_id(), game_guid.clone());
                                if tx.send(super::GameMessage::NewConnection { user, game_guid, connection, response, sender }).await.is_err() {
                                    log::error!("Failed to send NewConnection game message to existing match");
                                }
                            }
                        } else {
                            self.create_new_game(user, game_guid, connection, response, sender).await;
                        }
                    }
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
}
