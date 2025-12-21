pub struct ChatRoom {
    name: String,
    #[allow(dead_code)]
    channel: crate::data::channel::ChatChannelType,
    online_users: Vec<super::UserHandle>,
}

impl ChatRoom {
    /// Remove offline users that are still in the list
    fn cleanup(&mut self) -> usize {
        let mut total_changes = 0;
        let mut index = 0;
        while self.online_users.get(index).is_some() {
            if self.online_users.get(index).unwrap().is_online() {
                index += 1;
            } else {
                self.online_users.swap_remove(index);
                total_changes += 1;
            }
        }
        total_changes
    }

    pub fn is_empty(&self) -> bool {
        for user in self.online_users.iter() {
            if user.is_online() {
                return false;
            }
        }
        true
    }

    pub fn is_empty_mut(&mut self) -> bool {
        self.cleanup();
        self.is_empty()
    }

    pub fn send_public_message(&self, message: crate::events::chat_message::PublicMessage) {
        let user_id = &message.sender_display_name;
        let event = polariton::operation::Event {
            code: 1,
            params: message.as_event_params(),
        };
        for user in self.online_users.iter() {
            if user.name() == user_id { continue; }
            user.send(polariton_server::ToSend::Data {
                data: polariton::packet::Data::Event(event.clone()),
                      encrypt: true,
                      channel: 0,
                      reliable: true,
            });
        }
    }

    pub fn new(name: String, type_: crate::data::channel::ChatChannelType) -> Self {
        Self {
            name,
            channel: type_,
            online_users: Vec::new(),
        }
    }

    pub fn connect_user(&mut self, handle: super::UserHandle) {
        self.cleanup();
        let event = polariton::operation::Event {
            code: crate::events::room_join::RoomJoined::CODE,
            params: crate::events::room_join::RoomJoined {
                channel_name: self.name.clone(),
                player_name: handle.name().to_owned(),
                player_state: oj_rc_core::data::channel::ChatPlayerState::Idk0,
                use_custom_avatar: false,
                custom_avatar: Vec::default(),
                avatar_id: 0,
            }.as_event_params(),
        };
        handle.send(polariton_server::ToSend::Data {
            data: polariton::packet::Data::Event(event),
                    encrypt: true,
                    channel: crate::events::room_join::RoomJoined::CHANNEL,
                    reliable: true,
        });
        self.online_users.push(handle);
    }

    pub fn remove_user(&mut self, name: &str) -> bool {
        if let Some(user_index) = self.online_users.iter().position(|x| name == x.name()) {
            self.online_users.swap_remove(user_index);
            true
        } else {
            false
        }
    }

    pub fn canon_name(&self) -> String {
        self.name.clone()
    }
}
