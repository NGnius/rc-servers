use std::collections::HashMap;

#[derive(Clone)]
pub struct ChatProvider {
    chat_system: std::sync::Arc<tokio::sync::RwLock<crate::state::chat::ChatSystem>>,
}

impl ChatProvider {
    pub fn new(conf: oj_rc_core::persist::config::ChatSystemConfig) -> std::io::Result<Self> {
        Ok(Self {
            chat_system: std::sync::Arc::new(tokio::sync::RwLock::new(crate::state::chat::ChatSystem::new(conf)?)),
        })
    }

    pub async fn system(&self) -> tokio::sync::RwLockReadGuard<'_, crate::state::chat::ChatSystem> {
        self.chat_system.read().await
    }

    pub async fn system_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, crate::state::chat::ChatSystem> {
        self.chat_system.write().await
    }
}

pub struct ChatSystem {
    chats: HashMap<String, super::ChatRoom>,
    online_users: HashMap<String, super::UserHandle>,
    config: super::ChatSystemConfig,
}

impl ChatSystem {
    fn cleanup(&mut self) -> usize {
        let mut to_be_removed = Vec::new();
        for (key, val) in self.online_users.iter() {
            if !val.is_online() {
                to_be_removed.push(key.to_owned());
            }
        }
        for offline_user in to_be_removed.iter() {
            self.online_users.remove(offline_user);
        }
        let total_removed_users = to_be_removed.len();

        to_be_removed.clear();
        for (key, val) in self.chats.iter_mut() {
            if val.is_empty_mut() {
                to_be_removed.push(key.to_owned());
            }
        }
        for empty_room in to_be_removed.iter() {
            self.chats.remove(empty_room);
        }

        total_removed_users + to_be_removed.len()
    }

    pub fn connect_user(&mut self, display_name: String, channels: Vec<String>, event_tx: tokio::sync::mpsc::UnboundedSender<polariton_server::ToSend>) {
        self.cleanup();
        let handle = super::UserHandle::from_strong_sender(event_tx, display_name.clone());
        self.online_users.insert(display_name, handle.clone());
        for channel in channels {
            if let Some(chat) = self.chats.get_mut(&channel) {
                chat.connect_user(handle.clone());
            } else {
                let mut new_room = super::ChatRoom::new(channel.clone(), crate::data::channel::ChatChannelType::Public);
                new_room.connect_user(handle.clone());
                self.chats.insert(channel, new_room);
            }
        }
    }

    pub fn join_channel(&mut self, display_name: String, channel: String, channel_ty: crate::data::channel::ChatChannelType) {
        if let Some(user_handle) = self.online_users.get(&display_name) {
            if let Some(chat_room) = self.chats.get_mut(&channel) {
                chat_room.connect_user(user_handle.to_owned());
            } else {
                let mut new_room = super::ChatRoom::new(channel.clone(), channel_ty);
                new_room.connect_user(user_handle.to_owned());
                self.chats.insert(channel, new_room);
            }
        }
        self.cleanup();
    }

    pub fn leave_channel(&mut self, display_name: String, channel: String) {
        if let Some(chat_room) = self.chats.get_mut(&channel) {
            chat_room.remove_user(&display_name);
        }
    }

    pub async fn handle_public_message(&self, user: &(dyn oj_rc_core::persist::user::User<()> + Send + Sync), text: String, channel: String, channel_ty: crate::data::channel::ChatChannelType) {
        if self.config.is_command_channel(&channel) {
            if let Some(user_handle) = self.online_users.get(user.public_id()) {
                self.handle_public_command(user, text, user_handle, channel, channel_ty);
            }
        } else if let Some(room) = self.chats.get(&channel) {
            let event_params = crate::events::chat_message::PublicMessage {
                sender_name: user.public_id().to_owned(),
                sender_display_name: user.display_name().to_owned(),
                text,
                is_dev: user.is_dev(),
                is_mod: user.is_mod(),
                is_admin: user.is_admin(),
                channel_name: channel,
                channel_ty,
            };
            room.send_public_message(event_params);
        } else {
            log::warn!("Got message for non-existent chat room {} (variant: {:?})", channel, channel_ty);
        }
    }

    fn handle_public_command(&self, user: &dyn oj_rc_core::persist::user::User<()>, text: String, handle: &super::UserHandle, channel: String, channel_ty: crate::data::channel::ChatChannelType) {
        let event_params = crate::events::chat_message::PublicMessage {
            sender_name: self.config.command_username().to_owned(),
            sender_display_name: self.config.command_username().to_owned(),
            text: self.config.perform_command(&text, self, user),
            is_dev: false,
            is_mod: false,
            is_admin: false,
            channel_name: channel,
            channel_ty,
        };
        tokio::spawn(Self::send_public_command_response(handle.to_owned(), event_params));
    }

    async fn send_public_command_response(handle: super::UserHandle, response: crate::events::chat_message::PublicMessage) {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        let event = polariton::operation::Event {
            code: 1,
            params: response.as_event_params(),
        };
        handle.send(polariton_server::ToSend::Data { data: polariton::packet::Data::Event(event), encrypt: true, channel: 0, reliable: true });
    }

    pub fn handle_private_message(&self, user: &dyn oj_rc_core::persist::user::User<()>, text: String, recipient: String) {
        if self.config.is_command_user(&recipient) {
            if let Some(user_handle) = self.online_users.get(user.public_id()) {
                self.handle_private_command(user, text, user_handle);
            }
        } else if let Some(recipient_handle) = self.online_users.get(&recipient) {
            let private_msg = crate::events::chat_message::PrivateMessage {
                sender_name: user.public_id().to_owned(),
                sender_display_name: user.public_id().to_owned(),
                text,
                is_dev: user.is_dev(),
                is_mod: user.is_mod(),
                is_admin: user.is_admin(),
            };
            recipient_handle.send_private_message(private_msg);
        }
    }

    fn handle_private_command(&self, user: &dyn oj_rc_core::persist::user::User<()>, text: String, handle: &super::UserHandle) {
        let event_params = crate::events::chat_message::PrivateMessage {
            sender_name: self.config.command_username().to_owned(),
            sender_display_name: self.config.command_username().to_owned(),
            text: self.config.perform_command(&text, self, user),
            is_dev: false,
            is_mod: false,
            is_admin: false,
        };
        tokio::spawn(Self::send_private_command_response(handle.to_owned(), event_params));
    }

    async fn send_private_command_response(handle: super::UserHandle, response: crate::events::chat_message::PrivateMessage) {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        handle.send_private_message(response);
    }

    pub fn new(config: oj_rc_core::persist::config::ChatSystemConfig) -> std::io::Result<Self> {
        Ok(Self {
            chats: HashMap::new(),
            online_users: HashMap::new(),
            config: super::ChatSystemConfig::from_persist(config)?,
        })
    }

    pub fn user_count(&self) -> usize {
        self.online_users.values().filter(|x| x.is_online()).count()
    }

    pub fn is_user_online(&self, display_name: &str) -> bool {
        self.config.is_command_user(display_name) || self.online_users.get(display_name).map(|x| x.is_online()).unwrap_or(false)
    }
}
