pub struct GameMatches {
    matches: std::collections::HashMap<String, tokio::sync::mpsc::Sender<super::GameMessage>>,
    routing: std::collections::HashMap<i32, String>, // user id to game guid
}

impl GameMatches {
    pub fn new() -> Self {
        Self {
            matches: std::collections::HashMap::new(),
            routing: std::collections::HashMap::new(),
        }
    }

    pub fn spawn(self) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(super::CHANNEL_BOUND);
        tokio::spawn(self.run(rx));
        tx
    }

    async fn start_new_match_engine(&self, _user: &Box<dyn oj_rc_core::persist::user::MultiplayerUser + Send + Sync + 'static>, guid: &str) -> tokio::sync::mpsc::Sender<super::GameMessage> {
        // TODO figure out gamemode and act accordingly
        let engine = super::GenericGamemodeEngine::new(guid.to_owned(), super::modes::EliminationLogic::new());
        engine.spawn()
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
        let tx = self.start_new_match_engine(&user, &game_guid).await;
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
                                } else {
                                    if tx.send(msg).await.is_err() {
                                        log::error!("Failed to route game message from user {} to match {}", user_id, guid);
                                    }
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
