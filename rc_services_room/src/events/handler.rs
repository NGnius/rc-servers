use oj_rc_core::persist::user::IntercomListener;
use oj_rc_core::persist::user::intercom::IntercomWebServiceUserMessage;

pub struct IntercomHandler {
    listener: IntercomListener<IntercomWebServiceUserMessage>,
    user: std::sync::Weak<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
    emitter: polariton_server::events::WeakEventEmitter<()>,
}

impl IntercomHandler {
    pub fn new(
        listener: IntercomListener<IntercomWebServiceUserMessage>,
        user: &std::sync::Arc<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
        emitter: &polariton_server::events::EventEmitter<()>,
    ) -> Self {
        Self {
            listener,
            user: std::sync::Arc::downgrade(user),
            emitter: emitter.to_owned().downgrade(),
        }
    }

    async fn run_loop(
        listener: IntercomListener<IntercomWebServiceUserMessage>,
        user: std::sync::Weak<Box<dyn oj_rc_core::persist::user::User<()> + Send + Sync>>,
        emitter: polariton_server::events::WeakEventEmitter<()>
    ) {
        use futures::StreamExt;
        let mut listener = listener.listen().await;
        while let Some(msg) = listener.next().await {
            match msg {
                Ok(msg) => {
                    if let Some(_user) = user.upgrade() {
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
        tokio::spawn(Self::run_loop(self.listener, self.user, self.emitter))
    }
}
