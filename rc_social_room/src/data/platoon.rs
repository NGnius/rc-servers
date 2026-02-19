#[derive(Debug, Clone)]
pub struct PlatoonMemberInfo {
    pub public_id: String,
    pub display_name: String,
    pub status: MemberStatus,
    pub added: i64, // timestamp
    pub avatar_id: i32,
    pub use_custom_avatar: bool,
}

impl PlatoonMemberInfo {
    pub(super) fn dump(&self, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut total = 1 + 8 + 4 + 1;
        total += oj_rc_core::data::write_str_for_binreader(&self.public_id, w)?;
        total += oj_rc_core::data::write_str_for_binreader(&self.display_name, w)?;
        w.write_all(&[self.status.as_u8()])?;
        w.write_all(&self.added.to_be_bytes())?;
        w.write_all(&self.avatar_id.to_be_bytes())?;
        w.write_all(&[self.use_custom_avatar as u8])?;
        Ok(total)
    }

    pub(super) fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let public_id = oj_rc_core::data::read_str_for_binwriter(r)?;
        let display_name = oj_rc_core::data::read_str_for_binwriter(r)?;
        let mut buf = [0u8; 1 + 8 + 4 + 1];
        r.read_exact(&mut buf)?;
        let status = MemberStatus::from_u8(buf[0]).ok_or_else(|| std::io::Error::other(format!("Invalid platoon member status {}", buf[0])))?;
        let added = i64::from_be_bytes([
            buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8],
        ]);
        let avatar_id = i32::from_be_bytes([
            buf[9], buf[10], buf[11], buf[12],
        ]);
        let use_custom_avatar = buf[13] != 0;
        Ok(Self {
            public_id,
            display_name,
            status,
            added,
            avatar_id,
            use_custom_avatar,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MemberStatus {
    Invited = 0,
    Ready = 1,
    InQueue = 2,
    InBattle = 3,
}

impl MemberStatus {
    #[inline]
    pub fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Self::Invited),
            1 => Some(Self::Ready),
            2 => Some(Self::InQueue),
            3 => Some(Self::InBattle),
            _ => None,
        }
    }

    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
