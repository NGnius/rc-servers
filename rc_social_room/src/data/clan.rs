#![allow(dead_code)]
use polariton::operation::Typed;

pub struct ClanMember {
    pub username: String,
    pub display_name: String,
    pub member_state: ClanMemberState,
    pub use_custom_avatar: bool,
    pub avatar_id: i32,
    pub rank: ClanMemberRank,
    pub is_online: bool,
    pub season_xp: i32,
}

impl ClanMember {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("userName".into()), Typed::Str(self.username.clone().into())),
            (Typed::Str("displayName".into()), Typed::Str(self.display_name.clone().into())),
            (Typed::Str("memberState".into()), Typed::Int(self.member_state as i32)),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar.into())),
            (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id)),
            (Typed::Str("rank".into()), Typed::Int(self.rank as i32)),
            (Typed::Str("isOnline".into()), Typed::Bool(self.is_online.into())),
            (Typed::Str("seasonXP".into()), Typed::Int(self.season_xp)),
        ].into())
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ClanMemberState {
    Idk0,
    Idk1,
    Idk2,
    // TODO figure out how many of these there are (and what they mean)
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ClanMemberRank {
    Member = 0,
    Officer = 1,
    Leader = 2,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ClanType {
    Open = 1,
    Closed = 2,
}

pub struct ClanInfo {
    pub clan_name: String,
    pub clan_description: String,
    pub clan_type: ClanType,
    pub clan_size: i32,
}

impl ClanInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("clanName".into()), Typed::Str(self.clan_name.clone().into())),
            (Typed::Str("clanDescription".into()), Typed::Str(self.clan_description.clone().into())),
            (Typed::Str("clanType".into()), Typed::Int(self.clan_type as i32)),
            (Typed::Str("clanSize".into()), Typed::Int(self.clan_size)),
        ].into())
    }
}
