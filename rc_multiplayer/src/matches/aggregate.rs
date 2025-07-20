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
        let engine = super::GenericGamemodeEngine::new(guid.to_owned());
        engine.spawn()
    }

    async fn run(mut self, mut rx: tokio::sync::mpsc::Receiver<super::GameMessage>) {
        log::info!("Match message router has started");
        while !rx.is_closed() {
            if let Some(msg) = rx.recv().await {
                log::debug!("Match message router got a message");
                match msg {
                    super::GameMessage::NewConnection { user, game_guid, connection, response, sender } => {
                        if let Some(tx) = self.matches.get(&game_guid) {
                            self.routing.insert(user.user_id(), game_guid.clone());
                            if tx.send(super::GameMessage::NewConnection { user, game_guid, connection, response, sender }).await.is_err() {
                                log::error!("Failed to send NewConnection game message to existing match");
                            }
                        } else {
                            // create a new match
                            log::info!("Creating new game {}", game_guid);
                            let tx = self.start_new_match_engine(&user, &game_guid).await;
                            self.matches.insert(game_guid.clone(), tx.clone());
                            self.routing.insert(user.user_id(), game_guid.clone());
                            if tx.send(super::GameMessage::NewConnection { user, game_guid, connection, response, sender }).await.is_err() {
                                log::error!("Failed to send NewConnection game message to new match");
                            }
                        }
                    }
                    msg => {
                        let user_id = msg.user_id();
                        if let Some(guid) = self.routing.get(&user_id) {
                            if let Some(tx) = self.matches.get(guid) {
                                if tx.is_closed() {
                                    self.matches.remove(guid);
                                    self.routing.remove(&user_id);
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
                    }
                }
            }
        }
        log::warn!("Match message router has completed");
    }
}
