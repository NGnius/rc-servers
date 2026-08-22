use oj_rc_core::persist::user::IntercomListener;
use oj_rc_core::persist::user::intercom::{IntercomWebServiceUserMessage, IntercomWebServiceWorkaroundMessage};

pub struct IntercomHandler {
    listener: IntercomListener<IntercomWebServiceUserMessage>,
    user: std::sync::Weak<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
    emitter: polariton_server::events::WeakEventEmitter<()>,
    keybind_workaround: std::sync::Arc<crate::workarounds::EditModeInputLockupWorkaround>,
    game_event_seq: std::sync::Arc<tokio::sync::Mutex<oj_rc_core::persist::config::GameEventSequence>>,
}

impl IntercomHandler {
    pub fn new(
        listener: IntercomListener<IntercomWebServiceUserMessage>,
        user: &std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
        emitter: &polariton_server::events::EventEmitter<()>,
        keybind_workaround: &std::sync::Arc<crate::workarounds::EditModeInputLockupWorkaround>,
        game_event_seq: &std::sync::Arc<tokio::sync::Mutex<oj_rc_core::persist::config::GameEventSequence>>,
    ) -> Self {
        Self {
            listener,
            user: std::sync::Arc::downgrade(user),
            emitter: emitter.to_owned().downgrade(),
            keybind_workaround: keybind_workaround.to_owned(),
            game_event_seq: game_event_seq.to_owned(),
        }
    }

    async fn run_loop(
        listener: IntercomListener<IntercomWebServiceUserMessage>,
        user: std::sync::Weak<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
        emitter: polariton_server::events::WeakEventEmitter<()>,
        keybind_workaround: std::sync::Arc<crate::workarounds::EditModeInputLockupWorkaround>,
        game_event_seq: std::sync::Arc<tokio::sync::Mutex<oj_rc_core::persist::config::GameEventSequence>>,
    ) {
        use futures::StreamExt;
        let mut listener = listener.listen().await;
        while let Some(msg) = listener.next().await {
            match msg {
                Ok(msg) => {
                    if let Some(user) = user.upgrade() {
                        match msg {
                            IntercomWebServiceUserMessage::DevMessage(msg) => {
                                let clear_event = super::DevMessage {
                                    message: " ".to_owned(),
                                    duration: 1,
                                };
                                emitter.emit(clear_event);
                                let event = super::DevMessage {
                                    message: msg.message,
                                    duration: msg.duration as i32,
                                };
                                emitter.emit(event);
                            },
                            IntercomWebServiceUserMessage::Maintenance(msg) => {
                                let event = super::MaintenanceMode {
                                    message: msg.message,
                                };
                                emitter.emit(event);
                            }
                            IntercomWebServiceUserMessage::Workaround(IntercomWebServiceWorkaroundMessage::KeybindLockout { }) => {
                                let session = keybind_workaround.add_user(user.account_id(), user.public_id().to_owned()).await;
                                let non_me = session.users.first().unwrap();
                                let event = super::CustomGameInvite {
                                    inviter_public_id: non_me.public_id.clone(),
                                    inviter_display_name: non_me.public_id.clone(),
                                    session: session.session_id,
                                    avatar_id: Some(6),
                                    invited_to_team_a: true,
                                };
                                emitter.emit(event);
                            },
                            IntercomWebServiceUserMessage::Workaround(IntercomWebServiceWorkaroundMessage::GameModeEventLock { event }) => {
                                let is_replaced = game_event_seq.lock().await.add_lockout(
                                    user.account_id(),
                                    oj_rc_core::persist::config::GameEvent {
                                        map: event.map.into_conf(),
                                        visibility: event.visibility.into_conf(),
                                        mode: event.mode.into_conf(),
                                        auto_heal: event.auto_heal,
                                    },
                                );
                                log::info!("Activated game event lockout for user {} (replaced? {})", user.public_id(), is_replaced);
                            },
                            IntercomWebServiceUserMessage::Workaround(IntercomWebServiceWorkaroundMessage::GameModeEventUnlock { }) => {
                                let is_existed = game_event_seq.lock().await.remove_lockout(user.account_id());
                                log::info!("Deactivated game event lockout for user {} (existed? {})", user.public_id(), is_existed);
                            }
                        }
                    } else {
                        break;
                    }
                },
                Err(e) => {
                    log::error!("Bad intercom message received: {}", e);
                }
            }

        }
    }

    pub fn run(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(Self::run_loop(self.listener, self.user, self.emitter, self.keybind_workaround, self.game_event_seq))
    }
}
