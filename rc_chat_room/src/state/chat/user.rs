#[derive(Clone)]
pub struct UserHandle {
    display_name: String,
    event_tx: tokio::sync::mpsc::WeakUnboundedSender<polariton_server::ToSend>,
}

impl UserHandle {
    pub fn is_online(&self) -> bool {
        self.event_tx.strong_count() != 0
    }

    pub fn from_strong_sender(event_tx: tokio::sync::mpsc::UnboundedSender<polariton_server::ToSend>, display_name: String) -> Self {
        Self {
            event_tx: event_tx.downgrade(),
            display_name,
        }
    }

    pub fn send(&self, to_send: polariton_server::ToSend) -> bool {
        if let Some(event_tx) = self.event_tx.upgrade() {
            event_tx.send(to_send).is_ok()
        } else {
            false
        }
    }

    /*pub fn send_later(&self, to_send: polariton_server::ToSend, wait: std::time::Duration) {
        tokio::spawn(Self::send_after(self.event_tx.clone(), to_send, wait));
    }

    async fn send_after(event_tx: tokio::sync::mpsc::WeakUnboundedSender<polariton_server::ToSend>, to_send: polariton_server::ToSend, wait: std::time::Duration) {
        tokio::time::sleep(wait).await;
        if let Some(event_tx) = event_tx.upgrade() {
            event_tx.send(to_send).unwrap_or_default();
        }
    }*/

    pub fn send_private_message(&self, message: crate::events::chat_message::PrivateMessage) {
        let event = polariton::operation::Event {
            code: 2,
            params: message.as_event_params(),
        };
        self.send(polariton_server::ToSend::Data {
            data: polariton::packet::Data::Event(event.clone()),
            encrypt: true,
            channel: 0,
            reliable: true,
        });
    }

    pub fn name(&self) -> &'_ str {
        &self.display_name
    }
}
