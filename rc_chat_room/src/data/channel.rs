use polariton::operation::{Typed, Arr};

pub struct ChatChannelInfo {
    pub channel_name: String,
    pub members: Vec<ChatChannelMember>,
    pub channel_ty: ChatChannelType,
}

impl ChatChannelInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("channelName".into()), Typed::Str(self.channel_name.clone().into())),
            (Typed::Str("members".into()), Typed::Arr(Arr {
                ty: 104, // hashtable
                items: self.members.iter().map(|x| x.as_transmissible()).collect(),
            })),
            (Typed::Str("channelType".into()), Typed::Int(self.channel_ty as _)),
        ].into())
    }
}

pub struct ChatChannelMember {
    pub name: String,
    pub use_custom_avatar: bool,
    pub state: ChatPlayerState,
    pub custom_avatar: Vec<u8>, // always PNG?
    pub avatar_id: i32,
}

impl ChatChannelMember {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("name".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar.into())),
            (Typed::Str("state".into()), Typed::Int(self.state as _)),
            if self.use_custom_avatar {
                (Typed::Str("customAvatar".into()), Typed::Bytes(self.custom_avatar.clone().into()))
            } else {
                (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id))
            },
        ].into())
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ChatChannelType {
    None = 0,
    Public = 1,
    Battle = 2,
    BattleTeam = 3,
    Platoon = 4,
    Custom = 5,
    Clan = 6,
    Private = 7,
    CustomGame = 8,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ChatPlayerState {
    Idk0 = 0,
    Idk1 = 1,
    Idk2 = 2,
    // FIXME
}
