use polariton::operation::Typed;

pub struct ClanInviteInfo {
    pub username: String,
    pub display_name: String,
    pub clan_name: String,
    pub clan_size: i32,
    pub use_custom_avatar: bool,
    pub avatar_id: i32,
}

impl ClanInviteInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("userName".into()), Typed::Str(self.username.clone().into())),
            (Typed::Str("displayName".into()), Typed::Str(self.display_name.clone().into())),
            (Typed::Str("clanName".into()), Typed::Str(self.clan_name.clone().into())),
            (Typed::Str("clanSize".into()), Typed::Int(self.clan_size)),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar.into())),
            (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id)),
        ].into())
    }
}
