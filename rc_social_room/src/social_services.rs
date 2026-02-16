/// Primarily keeps track of who is online so events can be sent to them.
pub struct SocialMesh {
    users: tokio::sync::RwLock<std::collections::HashMap<String, UserHandle>>,
}

struct UserHandle {
    emitter: polariton_server::events::WeakEventEmitter<crate::data::custom::CustomType>,
    is_alive: std::sync::atomic::AtomicBool,
}

impl SocialMesh {
    pub fn new() -> Self {
        Self {
            users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn send_event_to(&self, public_id: &str, event: impl polariton_server::events::IntoEvent<crate::data::custom::CustomType>) -> bool {
        let user_lock = self.users.read().await;
        if let Some(user_handle) = user_lock.get(public_id) {
            let is_success = user_handle.emitter.emit(event);
            user_handle.is_alive.swap(is_success, std::sync::atomic::Ordering::SeqCst);
            is_success
        } else {
            false
        }
    }

    pub async fn add_user(
        &self,
        public_id: String,
        emitter: polariton_server::events::WeakEventEmitter<crate::data::custom::CustomType>,
    ) {
        let mut user_lock = self.users.write().await;
        Self::cleanup_dead_users(&mut user_lock).await;
        user_lock.insert(public_id, UserHandle {
            emitter,
            is_alive: std::sync::atomic::AtomicBool::new(true),
        });
    }

    /// Filter out offline users
    pub async fn filter_online_only(&self, public_ids: &mut std::collections::HashSet<String>) {
        let mut user_lock = self.users.write().await;
        Self::cleanup_dead_users(&mut user_lock).await;
        public_ids.retain(|public_id| user_lock.contains_key(public_id));
    }

    async fn cleanup_dead_users(users: &mut std::collections::HashMap<String, UserHandle>) {
        users.retain(|_public_id, handle| handle.is_alive.load(std::sync::atomic::Ordering::SeqCst));
    }

    pub async fn online_count_read(&self) -> u64 {
        let user_lock = self.users.read().await;
        user_lock.iter()
            .filter(|(_, handle)| handle.is_alive.load(std::sync::atomic::Ordering::SeqCst))
            .count() as u64
    }
}
