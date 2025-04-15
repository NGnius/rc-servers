pub struct ChatRoom {
    name: String,
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
        let event = polariton::operation::Event {
            code: 1,
            params: message.as_event_params(),
        };
        for user in self.online_users.iter() {
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
            code: 1,
            params: crate::events::chat_message::PublicMessage {
                sender_name: "system".to_owned(),
                sender_display_name: "system".to_owned(),
                channel_name: self.name.clone(),
                channel_ty: self.channel,
                text: "joined".to_owned(),
                is_dev: false,
                is_mod: false,
                is_admin: false,
            }.as_event_params(),
        };
        handle.send(polariton_server::ToSend::Data {
            data: polariton::packet::Data::Event(event),
                    encrypt: true,
                    channel: 0,
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
}
