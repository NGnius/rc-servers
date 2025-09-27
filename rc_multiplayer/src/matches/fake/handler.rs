enum Message {
    Ready {
        real_players: std::collections::HashMap<u8, crate::matches::generic::UserSender>,
    },
    ClientMap {
        responder: tokio::sync::oneshot::Sender<std::collections::HashMap<u8, Vec<u8>>>, // real player_id -> client AIs
    },
    Stop,
}

pub struct Handler {
    tx: tokio::sync::mpsc::UnboundedSender<Message>,
}

impl Handler {
    pub async fn start(fakes: std::collections::HashMap<u8, Box<dyn super::FakeUser + 'static>>, descriptors: Vec<oj_rc_core::persist::user::PlayerDescriptor>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::task::spawn(handler_loop(rx, fakes, descriptors));
        Self {
            tx,
        }
    }

    pub fn on_ready(&self, real_players: std::collections::HashMap<u8, crate::matches::generic::UserSender>) {
        Self::log_failure(self.tx.send(Message::Ready { real_players }));
    }

    /// real player_id -> list of non-user player_id
    pub async fn get_client_ais(&self) -> std::collections::HashMap<u8, Vec<u8>> {
        let (responder, rx) = tokio::sync::oneshot::channel();
        Self::log_failure(self.tx.send(Message::ClientMap { responder }));
        rx.await.unwrap_or_default()
    }

    pub fn stop(&self) {
        Self::log_failure(self.tx.send(Message::Stop));
    }

    #[inline]
    fn log_failure<T>(res: Result<(), tokio::sync::mpsc::error::SendError<T>>) {
        if let Err(_e) = res {
            log::warn!("Failed to send message to fake user handler thread");
        }
    }
}

async fn handler_loop(mut rx: tokio::sync::mpsc::UnboundedReceiver<Message>, players: std::collections::HashMap<u8, Box<dyn super::FakeUser + 'static>>, descriptors: Vec<oj_rc_core::persist::user::PlayerDescriptor>) {
    for player in descriptors.iter() {
        if player.user_id.is_none() {
            let player_id = player.player_id;
            if let Some(fake_player) = players.get(&player_id) {
                fake_player.on_init(&descriptors, player_id).await;
            } else {
                log::warn!("No fake user implementation for player {}", player_id);
            }
        }
    }
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Ready { real_players } => {
                for fake in players.values() {
                    fake.on_ready(&real_players).await;
                }
            },
            Message::ClientMap { responder } => {
                let mut map = std::collections::HashMap::<u8, Vec<u8>>::new();
                for (player_id, fake) in players.iter() {
                    if let Some(running_on) = fake.running_on().await {
                        if let Some(fakes_on) = map.get_mut(&running_on) {
                            fakes_on.push(*player_id);
                        } else {
                            map.insert(running_on, vec![*player_id]);
                        }
                    }
                }
                if let Err(_e) = responder.send(map) {
                    log::warn!("Failed to send response for client AI map message");
                }
            },
            Message::Stop => {
                for fake in players.values() {
                    fake.on_end().await;
                }
                break;
            },
        }
    }
}
