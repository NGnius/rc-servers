pub struct UserMesh {
    online_users: tokio::sync::RwLock<std::collections::HashMap<String, UserHandle>>,
}

struct UserHandle {
    emitter: polariton_server::events::WeakEventEmitter,
    is_alive: std::sync::atomic::AtomicBool,
}

impl UserMesh {
    pub fn new() -> Self {
        Self {
            online_users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn user_count(&self) -> usize {
        self.online_users.read().await.values()
            .filter(|u| u.is_alive.load(std::sync::atomic::Ordering::Relaxed))
            .count()
    }

    /// returns whether the user was replaced (true) or new (false)
    pub async fn add_user(
        &self,
        public_id: String,
        emitter: polariton_server::events::WeakEventEmitter,
    ) -> bool {
        let handle = UserHandle {
            emitter,
            is_alive: std::sync::atomic::AtomicBool::new(true),
        };
        self.online_users.write().await
            .insert(public_id, handle)
            .is_some()
    }

    /// returns whether the user existing (true) or not (false)
    pub async fn remove_user(
        &self,
        public_id: String,
    ) -> bool {
        self.online_users.write().await
            .remove(&public_id)
            .is_some()
    }

    pub async fn broadcast_event_to(&self, public_ids: impl std::iter::Iterator<Item = &str>, event: impl polariton_server::events::IntoEvent<()> + Clone) -> bool {
        let user_lock = self.online_users.read().await;
        let mut total_success = true;
        for public_id in public_ids {
            let is_success = if let Some(user_handle) = user_lock.get(public_id) {
                let is_success = user_handle.emitter.emit(event.clone());
                user_handle.is_alive.swap(is_success, std::sync::atomic::Ordering::SeqCst);
                is_success
            } else {
                false
            };
            total_success &= is_success;
        }
        total_success
    }

    pub async fn send_event_to(&self, public_id: &str, event: impl polariton_server::events::IntoEvent<()>) -> bool {
        let user_lock = self.online_users.read().await;
        if let Some(user_handle) = user_lock.get(public_id) {
            let is_success = user_handle.emitter.emit(event);
            user_handle.is_alive.swap(is_success, std::sync::atomic::Ordering::SeqCst);
            is_success
        } else {
            false
        }
    }

    pub async fn is_user_online(&self, public_id: &str) -> bool {
        if let Some(user) = self.online_users.read().await.get(public_id) {
            user.is_alive.load(std::sync::atomic::Ordering::Relaxed)
        } else {
            false
        }
    }
}
