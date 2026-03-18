//use oj_rc_core::persist::user::IntercomListener;
use oj_rc_core::persist::user::intercom::IntercomLobbyStateMessage;

const RETRY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);

pub struct IntercomHandler<U: oj_rc_core::persist::user::Userless> {
    userless: std::sync::Arc<U>,
    lobby: std::sync::Arc<crate::QueueHandler>,
}

impl <U: oj_rc_core::persist::user::Userless + 'static> IntercomHandler<U> {
    pub fn new(
        userless: std::sync::Arc<U>,
        lobby: std::sync::Arc<crate::QueueHandler>,
    ) -> Self {
        Self {
            userless,
            lobby,
        }
    }

    async fn run_loop(
        userless:  std::sync::Arc<U>,
        lobby: std::sync::Arc<crate::QueueHandler>,
    ) {
        loop {
            let listener = match userless.lobby_state_listener().await {
                Ok(listener) => {
                    log::debug!("Connected lobby state intercom listener");
                    listener
                },
                Err(e) => {
                    log::error!("Failed to connect to lobby state intercom: {} (retrying in {}s)", e, RETRY_TIMEOUT.as_secs());
                    tokio::time::sleep(RETRY_TIMEOUT).await;
                    continue;
                }
            };
            use futures::StreamExt;
            let mut listener = listener.listen().await;
            while let Some(msg) = listener.next().await {
                match msg {
                    Ok(msg) => {
                        match msg {
                            IntercomLobbyStateMessage::CustomGame(state) => {
                                if state.users.is_empty() {
                                    // disband
                                    lobby.remove_custom_queue(&state.session_id).await;
                                } else {
                                    let is_create = lobby.update_custom_queue(
                                        &state.session_id,
                                        state.users.iter().map(|user| (user.public_id.clone(), user.team)),
                                        state.config,
                                    ).await;
                                    if is_create {
                                        log::debug!("Created custom game {} lobby data to {} members", state.session_id, state.users.len());
                                    } else {
                                        log::debug!("Updated custom game {} lobby data to {} members", state.session_id, state.users.len());
                                    }
                                }
                            },
                        }
                    },
                    Err(e) => {
                        log::error!("Bad intercom message received: {}", e);
                    }
                }
            }
        }
    }

    pub fn run(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(Self::run_loop(self.userless, self.lobby))
    }
}
