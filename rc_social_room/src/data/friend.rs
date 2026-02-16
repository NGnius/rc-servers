use polariton::operation::Typed;

pub struct AvatarInfo {
    pub name: String,
    pub use_custom_avatar: bool,
    pub avatar_id: i32,
}

impl AvatarInfo {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("name".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar)),
            (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id)),
        ].into())
    }
}

#[derive(Debug, Clone)]
pub struct FriendInfo {
    pub status: InviteStatus,
    pub is_online: bool,
    pub public_id: String,
    pub display_name: String,
    pub clan_name: String,
}

impl FriendInfo {
    pub(super) fn dump(&self, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        w.write_all(&[
            self.status.as_u8(),
            self.is_online as u8,
        ])?;
        let mut total = 2;
        total += oj_rc_core::data::write_str_for_binreader(&self.public_id, w)?;
        total += oj_rc_core::data::write_str_for_binreader(&self.display_name, w)?;
        total += oj_rc_core::data::write_str_for_binreader(&self.clan_name, w)?;
        Ok(total)
    }

    pub(super) fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut buf = [0u8; 2];
        r.read_exact(&mut buf)?;
        let status = InviteStatus::from_u8(buf[0]).ok_or_else(|| std::io::Error::other(format!("Invalid invite status {}", buf[0])))?;
        let is_online = buf[1] != 0;
        let public_id = oj_rc_core::data::read_str_for_binwriter(r)?;
        let display_name = oj_rc_core::data::read_str_for_binwriter(r)?;
        let clan_name = oj_rc_core::data::read_str_for_binwriter(r)?;
        Ok(Self {
            status,
            is_online,
            public_id,
            display_name,
            clan_name,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InviteStatus {
    InviteSent = 0,
    InvitePending = 1,
    Accepted = 2,
    None = 3
}

impl InviteStatus {
    #[inline]
    pub fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::InviteSent),
            1 => Some(Self::InvitePending),
            2 => Some(Self::Accepted),
            3 => Some(Self::None),
            _ => None,
        }
    }

    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_core(core: &oj_rc_core::persist::user::FriendInviteStatus) -> Self {
        match core {
            oj_rc_core::persist::user::FriendInviteStatus::InviteSent => Self::InviteSent,
            oj_rc_core::persist::user::FriendInviteStatus::InvitePending => Self::InvitePending,
            oj_rc_core::persist::user::FriendInviteStatus::Accepted => Self::Accepted,
            oj_rc_core::persist::user::FriendInviteStatus::Declined
            | oj_rc_core::persist::user::FriendInviteStatus::Cancelled
            | oj_rc_core::persist::user::FriendInviteStatus::Removed => Self::None,
        }
    }

    #[inline]
    pub fn reciprocal(&self) -> Self {
        match self {
            Self::InviteSent => Self::InvitePending,
            Self::InvitePending => Self::InviteSent,
            Self::Accepted => Self::Accepted,
            Self::None => Self::None,
        }
    }
}
