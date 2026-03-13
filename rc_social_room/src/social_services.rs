/// Primarily keeps track of who is online so events can be sent to them.
pub struct SocialMesh {
    users: tokio::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<UserHandle>>>,
    platoons: Platoons,
}

struct UserHandle {
    emitter: polariton_server::events::WeakEventEmitter<crate::data::custom::CustomType>,
    is_alive: std::sync::atomic::AtomicBool,
}

struct Platoons {
    platoon_by_id: tokio::sync::RwLock<std::collections::HashMap<String, Vec<PlatoonMember>>>,
    platoon_by_user: tokio::sync::RwLock<std::collections::HashMap<String, String>>, // public_id -> platoon_id
}

struct PlatoonMember {
    public_id: String,
    //handle: std::sync::Arc<UserHandle>,
    status: crate::data::platoon::MemberStatus,
    timestamp: i64,
}

pub struct PlatoonMemberInfo {
    pub public_id: String,
    pub status: crate::data::platoon::MemberStatus,
    pub timestamp: i64,
}

fn platoon_key(creator: &str) -> String {
    let now = chrono::Utc::now().timestamp();
    format!("{}_{}_p", creator, now)
}

impl SocialMesh {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            users: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            platoons: Platoons {
                platoon_by_id: tokio::sync::RwLock::new(std::collections::HashMap::new()),
                platoon_by_user: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            },
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
        user_lock.insert(public_id, std::sync::Arc::new(UserHandle {
            emitter,
            is_alive: std::sync::atomic::AtomicBool::new(true),
        }));
    }

    /// Filter out offline users
    pub async fn filter_online_only(&self, public_ids: &mut std::collections::HashSet<String>) {
        let mut user_lock = self.users.write().await;
        Self::cleanup_dead_users(&mut user_lock).await;
        public_ids.retain(|public_id| user_lock.contains_key(public_id));
    }

    async fn cleanup_dead_users(users: &mut std::collections::HashMap<String, std::sync::Arc<UserHandle>>) {
        users.retain(|_public_id, handle| handle.is_alive.load(std::sync::atomic::Ordering::SeqCst));
    }

    pub async fn online_count_read(&self) -> u64 {
        let user_lock = self.users.read().await;
        user_lock.iter()
            .filter(|(_, handle)| handle.is_alive.load(std::sync::atomic::Ordering::SeqCst))
            .count() as u64
    }

    pub async fn platoon_of_user(&self, public_id: &str) -> Option<String> {
        self.platoons.platoon_by_user.read().await
            .get(public_id)
            .map(|x| x.to_owned())
    }

    pub async fn users_of_platoon(&self, platoon_id: &str) -> Vec<PlatoonMemberInfo> {
        self.platoons.platoon_by_id.read().await
            .get(platoon_id)
            .map(|plat| plat.iter()
                .map(|member| PlatoonMemberInfo {
                    public_id: member.public_id.clone(),
                    status: member.status,
                    timestamp: member.timestamp,
                })
                .collect()
            ).unwrap_or_default()
    }

    pub async fn add_user_to_platoon(&self, public_id: &str, platoon_id: &str, status: crate::data::platoon::MemberStatus) -> Option<i64> {
        /*let user_handle = if let Some(handle) = self.users.read().await.get(public_id) {
            handle.to_owned()
        } else {
            return None;
        };*/
        if let Some(platoon) = self.platoons.platoon_by_id.write().await.get_mut(platoon_id) {
            let timestamp = chrono::Utc::now().timestamp();
            platoon.push(PlatoonMember {
                public_id: public_id.to_owned(),
                //handle: user_handle,
                status,
                timestamp,
            });
            self.platoons.platoon_by_user.write().await.insert(public_id.to_owned(), platoon_id.to_owned());
            Some(timestamp)
        } else {
            None
        }
    }

    pub async fn create_platoon(&self, public_id: &str) -> Option<(i64, String)> {
        /*let user_handle = if let Some(handle) = self.users.read().await.get(public_id) {
            handle.to_owned()
        } else {
            return None;
        };*/
        let platoon_id = platoon_key(public_id);
        if self.platoons.platoon_by_id.read().await.contains_key(&platoon_id) {
            return None;
        }
        let mut platoon_members = Vec::with_capacity(5);
        let timestamp = chrono::Utc::now().timestamp();
        platoon_members.push(PlatoonMember {
            public_id: public_id.to_owned(),
            //handle: user_handle,
            status: crate::data::platoon::MemberStatus::Ready,
            timestamp,
        });
        self.platoons.platoon_by_id.write().await.insert(platoon_id.to_owned(), platoon_members);
        self.platoons.platoon_by_user.write().await.insert(public_id.to_owned(), platoon_id.to_owned());
        Some((timestamp, platoon_id))
    }

    pub async fn remove_user_from_platoon(&self, public_id: &str) -> bool {
        if let Some(platoon_id) = self.platoons.platoon_by_user.write().await.remove(public_id) {
            let mut platoon_by_id_lock = self.platoons.platoon_by_id.write().await;
            if let Some(platoon) = platoon_by_id_lock.get_mut(&platoon_id) {
                platoon.retain(|member| member.public_id != public_id);
                if platoon.is_empty() {
                    platoon_by_id_lock.remove(&platoon_id);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub async fn update_user_in_platoon(&self, public_id: &str, status: crate::data::platoon::MemberStatus) -> bool {
        if let Some(platoon_id) = self.platoons.platoon_by_user.write().await.get_mut(public_id) {
            if let Some(platoon) = self.platoons.platoon_by_id.write().await.get_mut(platoon_id) {
                for member in platoon.iter_mut() {
                    if member.public_id != public_id { continue; }
                    member.status = status;
                    break;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub async fn remove_platoon(&self, platoon_id: &str) -> bool {
        if let Some(platoon) = self.platoons.platoon_by_id.write().await.remove(platoon_id) {
            let mut platoon_by_user_lock = self.platoons.platoon_by_user.write().await;
            for member in platoon {
                platoon_by_user_lock.remove(&member.public_id);
            }
            true
        } else {
            false
        }
    }
}
