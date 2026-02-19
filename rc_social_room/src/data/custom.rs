#[derive(Debug, Clone)]
pub enum CustomType {
    FriendInfo(super::friend::FriendInfo),
    PlatoonMember(super::platoon::PlatoonMemberInfo),
    Unknown,
}

impl CustomType {
    fn custom_ty(&self) -> u8 {
        match self {
            Self::FriendInfo(_) => 0,
            Self::PlatoonMember(_) => 1,
            Self::Unknown => u8::MAX,
        }
    }
}

pub struct CustomTypeSerdes;

impl polariton::serdes::CustomSerdes<CustomType> for CustomTypeSerdes {
    fn custom_ty(c: &CustomType) -> u8 {
        c.custom_ty()
    }

    fn dump_inner(c: &CustomType, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let total_written_len = match c {
            CustomType::FriendInfo(friend) => {
                friend.dump(w)?
            },
            CustomType::PlatoonMember(member) => {
                member.dump(w)?
            },
            CustomType::Unknown => 0,
        };
        Ok(total_written_len)
    }

    fn parse_inner(ty: u8, r: &mut dyn std::io::Read) -> std::io::Result<CustomType> {
        match ty {
            0 => super::friend::FriendInfo::parse(r).map(CustomType::FriendInfo),
            1 => super::platoon::PlatoonMemberInfo::parse(r).map(CustomType::PlatoonMember),
            _ => Ok(CustomType::Unknown),
        }
    }
}
