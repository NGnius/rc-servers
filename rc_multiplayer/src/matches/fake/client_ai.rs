pub struct ClientAIPlayer {
    me: oj_rc_core::persist::user::PlayerDescriptor,
    is_complete: std::sync::Arc<std::sync::atomic::AtomicBool>,
    assigned_to: std::sync::atomic::AtomicU16, // player_id but where u16::MAX means None
}

impl ClientAIPlayer {
    pub fn new(me: oj_rc_core::persist::user::PlayerDescriptor) -> Self {
        Self {
            me,
            is_complete: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            assigned_to: std::sync::atomic::AtomicU16::new(u16::MAX),
        }
    }

    fn assigned_to_player_id(&self) -> Option<u8> {
        let id = self.assigned_to.load(std::sync::atomic::Ordering::Relaxed);
        if id > u8::MAX as u16 {
            None
        } else {
            Some(id as u8)
        }
    }

    fn set_assigned_to(&self, player_id: Option<u8>) {
        self.assigned_to.store(player_id.map(|x| x as u16).unwrap_or(u16::MAX), std::sync::atomic::Ordering::Relaxed);
    }
}

#[async_trait::async_trait]
impl super::FakeUser for ClientAIPlayer {
    async fn on_init(&self, descriptors: &[oj_rc_core::persist::user::PlayerDescriptor], player_id: u8) {
        let first_fake_i = descriptors.iter()
            .filter(|x| x.team == self.me.team)
            .enumerate()
            .find(|x| x.1.mode != None)
            .map(|(i, _)| i)
            .unwrap();
        let my_i = descriptors.iter()
            .enumerate()
            .find(|x| x.1.player_id == player_id)
            .map(|(i, _)| i)
            .unwrap();
        let real_teammates_count = descriptors.iter()
            .filter(|x| x.team == self.me.team && x.mode.is_none())
            .count();
        let my_offset = (my_i - first_fake_i) % real_teammates_count;
        for (i, teammate) in descriptors.iter().filter(|x| x.team == self.me.team && x.mode.is_none()).enumerate() {
            if i >= my_offset {
                self.set_assigned_to(Some(teammate.player_id));
                break;
            }
        }
        if self.assigned_to_player_id().is_none() {
            log::warn!("Failed to assign client AI player {} to a real client; offset:{}, first_fake:{}, reals:{}, me:{}", player_id, my_offset, first_fake_i, real_teammates_count, my_i);
        }
    }

    async fn on_ready(&self, _real_players: &std::collections::HashMap<u8, crate::matches::generic::UserSender>) {

    }

    async fn on_end(&self) {
        self.is_complete.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    async fn running_on(&self) -> Option<u8> {
        self.assigned_to_player_id()
    }
}
