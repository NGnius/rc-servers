use serde::{Serialize, Deserialize};
use polariton::operation::{Typed, Arr};

use crate::data::channel::{ChatChannelInfo, ChatChannelType};

pub struct ChatUserInfo {
    data: std::sync::RwLock<ChatUserData>,
    root: std::path::PathBuf,
}

impl ChatUserInfo {
    pub fn load(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let data = ChatUserData::load(root.as_ref())?;
        Ok(Self {
            data: std::sync::RwLock::new(data),
            root: root.as_ref().to_path_buf(),
        })
    }

    /*pub fn save(&self) -> std::io::Result<()> {
        self.data.read().unwrap().save(&self.root)
    }*/

    pub fn default_load(root: impl AsRef<std::path::Path>) -> Self {
        let data = ChatUserData::default_load();
        if let Err(e) = data.save(root.as_ref()) {
            log::error!("Failed to save default chat data to {}: {}", root.as_ref().display(), e);
        }
        Self {
            data: std::sync::RwLock::new(data),
            root: root.as_ref().to_path_buf(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatUserData {
    subscribed_channels: Vec<String>,
}

impl ChatUserData {
    fn load(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = std::fs::File::open(root.as_ref().join(super::CHAT_USER_FILE))?;
        let buffered = std::io::BufReader::new(file);
        let result = serde_json::from_reader(buffered)?;
        Ok(result)
    }

    fn save(&self, root: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let file = std::fs::File::create(root.as_ref().join(super::CHAT_USER_FILE))?;
        let buffered = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(buffered, self)?;
        Ok(())
    }

    fn default_load() -> Self {
        Self {
            subscribed_channels: vec![
                "main".to_owned(),
                "sys".to_owned(),
            ]
        }
    }
}

impl super::ChatUser for ChatUserInfo {
    fn subscribed_channels(&self) -> polariton::operation::Typed<()> {
        Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashtable
            items: self.data.read().unwrap().subscribed_channels.iter().map(|name| ChatChannelInfo {
                channel_name: name.to_owned(),
                members: Vec::default(),
                channel_ty: ChatChannelType::Public,
            }.as_transmissible()).collect()
        })
    }

    fn subscribed_channels_strings(&self) -> Vec<String> {
        self.data.read().unwrap().subscribed_channels.clone()
    }

    fn add_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Typed<()> {
        if let crate::data::channel::ChatChannelType::Public = channel_ty {
            let mut lock = self.data.write().unwrap();
            lock.subscribed_channels.push(channel.clone());
            if let Err(e) = lock.save(&self.root) {
                log::error!("Failed to save chat data to {}: {}", self.root.display(), e);
            }
        }
        crate::data::channel::ChatChannelInfo {
            channel_name: channel,
            members: Vec::default(),
            channel_ty,
        }.as_transmissible()
    }

    fn remove_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> bool {
        if let crate::data::channel::ChatChannelType::Public = channel_ty {
            let mut lock = self.data.write().unwrap();
            if let Some(index) = lock.subscribed_channels.iter().position(|chann| chann == &channel) {
                lock.subscribed_channels.swap_remove(index);
                if let Err(e) = lock.save(&self.root) {
                    log::error!("Failed to save chat data to {}: {}", self.root.display(), e);
                } else {
                    return true;
                }
            }
        }
        false
    }
}
