#![allow(unused)]
use polariton::operation::Typed;

pub struct ClanInviteInfo {
    pub username: oj_rc_core::types::PublicId,
    pub display_name: oj_rc_core::types::DisplayName,
    pub clan_name: String,
    pub clan_size: i32,
    pub use_custom_avatar: bool,
    pub avatar_id: i32,
}

impl ClanInviteInfo {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        use oj_rc_core::types::RawType;
        Typed::HashMap(vec![
            (Typed::Str("userName".into()), Typed::Str(self.username.as_raw_ref().into())),
            (Typed::Str("displayName".into()), Typed::Str(self.display_name.as_raw_ref().into())),
            (Typed::Str("clanName".into()), Typed::Str(self.clan_name.clone().into())),
            (Typed::Str("clanSize".into()), Typed::Int(self.clan_size)),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar)),
            (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id)),
        ].into())
    }

    pub fn from_core(core: oj_rc_core::persist::user::ClanInviteData) -> Self {
        Self {
            username: core.public_id,
            display_name: core.display_name,
            clan_name: core.clan_name,
            clan_size: core.size,
            use_custom_avatar: core.avatar_id.is_none(),
            avatar_id: core.avatar_id.unwrap_or_default(),
        }
    }
}
