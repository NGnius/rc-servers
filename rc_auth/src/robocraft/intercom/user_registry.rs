use oj_rc_core::persist::user::intercom::IntercomWebServiceUserMessage;

pub struct Users {
    service_listeners: tokio::sync::RwLock<std::collections::HashMap<String, tokio::sync::mpsc::Sender<IntercomWebServiceUserMessage>>>,
}

impl Users {
    pub fn new() -> Self {
        Self {
            service_listeners: tokio::sync::RwLock::new(std::collections::HashMap::with_capacity(16)),
        }
    }

    pub async fn register_service(&self, public_id: String, sender: tokio::sync::mpsc::Sender<IntercomWebServiceUserMessage>) {
        let mut write_lock = self.service_listeners.write().await;
        if let Some(old_sender) = write_lock.insert(public_id.clone(), sender) {
            if !old_sender.is_closed() {
                log::warn!("Replaced web services intercom channel for user {} (why duplicate!?)", public_id);
            }
        }
    }

    pub async fn remove_service(&self, public_id: String) {
        let mut write_lock = self.service_listeners.write().await;
        if write_lock.remove(&public_id).is_none() {
            log::warn!("Tried to remove web services intercom channel for user {} without listener", public_id);
        }
    }

    pub async fn broadcast_service_message(&self, msg: oj_rc_core::persist::user::intercom::IntercomWebServiceMessage) {
        let read_lock = self.service_listeners.read().await;
        for public_id in msg.public_ids {
            if let Some(tx) = read_lock.get(&public_id) {
                if let Err(e) = tx.send(msg.data.clone()).await {
                    log::error!("Failed to send web service intercom message to {}: {}", public_id, e);
                }
            } else {
                log::warn!("Not sending web service intercom message to user {}; no listener found", public_id);
            }
        }
    }
}
