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
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("userName".into()), Typed::Str(self.username.clone().into())),
            (Typed::Str("displayName".into()), Typed::Str(self.display_name.clone().into())),
            (Typed::Str("memberState".into()), Typed::Int(self.member_state as i32)),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar)),
            (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id)),
            (Typed::Str("rank".into()), Typed::Int(self.rank as i32)),
            (Typed::Str("isOnline".into()), Typed::Bool(self.is_online)),
            (Typed::Str("seasonXP".into()), Typed::Int(self.season_xp)),
        ].into())
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ClanMemberState {
    Invited = 0, // Invited
    Confirmed = 1, // Confirmed
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ClanMemberRank {
    Member = 0,
    Officer = 1,
    Leader = 2,
}

impl ClanMemberRank {
    #[inline]
    pub fn from_core(rank: oj_rc_core::persist::user::ClanMemberRank) -> Self {
        match rank {
            oj_rc_core::persist::user::ClanMemberRank::Member => Self::Member,
            oj_rc_core::persist::user::ClanMemberRank::Officer => Self::Officer,
            oj_rc_core::persist::user::ClanMemberRank::Leader => Self::Leader,
        }
    }

    #[inline]
    pub fn to_core(self) -> oj_rc_core::persist::user::ClanMemberRank {
        match self {
            Self::Member => oj_rc_core::persist::user::ClanMemberRank::Member,
            Self::Officer => oj_rc_core::persist::user::ClanMemberRank::Officer,
            Self::Leader => oj_rc_core::persist::user::ClanMemberRank::Leader,
        }
    }

    pub fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::Member),
            1 => Some(Self::Officer),
            2 => Some(Self::Leader),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ClanType {
    Open = 0,
    Closed = 1,
}

impl ClanType {
    #[inline]
    pub fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::Open),
            1 => Some(Self::Closed),
            _ => None
        }
    }

    #[inline]
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn to_core(self) -> oj_rc_core::persist::user::ClanType {
        match self {
            Self::Open => oj_rc_core::persist::user::ClanType::Open,
            Self::Closed => oj_rc_core::persist::user::ClanType::Closed,
        }
    }

    #[inline]
    pub fn from_core(ty: oj_rc_core::persist::user::ClanType) -> Self {
        match ty {
            oj_rc_core::persist::user::ClanType::Open => Self::Open,
            oj_rc_core::persist::user::ClanType::Closed => Self::Closed,
        }
    }
}

pub struct ClanInfo {
    pub clan_name: String,
    pub clan_description: String,
    pub clan_type: ClanType,
    pub clan_size: i32,
}

impl ClanInfo {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("clanName".into()), Typed::Str(self.clan_name.clone().into())),
            (Typed::Str("clanDescription".into()), Typed::Str(self.clan_description.clone().into())),
            (Typed::Str("clanType".into()), Typed::Int(self.clan_type as i32)),
            (Typed::Str("clanSize".into()), Typed::Int(self.clan_size)),
        ].into())
    }
}
