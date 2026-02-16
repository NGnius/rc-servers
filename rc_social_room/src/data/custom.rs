#[derive(Debug, Clone)]
pub enum CustomType {
    FriendInfo(super::friend::FriendInfo), // TODO actually serialise
    Unknown,
}

impl CustomType {
    fn custom_ty(&self) -> u8 {
        match self {
            Self::FriendInfo(_) => 0,
            Self::Unknown => 1,
        }
    }
}

pub struct CustomTypeSerdes;

impl polariton::serdes::CustomSerdes<CustomType> for CustomTypeSerdes {
    fn dump(c: &CustomType, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        w.write_all(&[c.custom_ty()])?;
        let mut buf = Vec::new();
        let total_written_len = match c {
            CustomType::FriendInfo(friend) => {
                friend.dump(&mut std::io::Cursor::new(&mut buf))?
            },
            CustomType::Unknown => 0,
        };
        w.write_all(&(total_written_len as i16).to_be_bytes())?;
        w.write_all(&buf)?;
        Ok(3 + total_written_len)
        /*let payload = vec![ // FIXME don't manually serialize
            0u8, // byte custom type
            0u8, 5u8, // short custom object size
            3u8, 0u8, 0u8, 0u8, 0u8, // content
        ];*/
    }

    fn parse(r: &mut dyn std::io::Read) -> std::io::Result<CustomType> {
        let mut buf = [0u8; 3];
        r.read_exact(&mut buf)?;
        // TODO only read up up to size
        match buf[0] {
            0 => super::friend::FriendInfo::parse(r).map(CustomType::FriendInfo),
            _ => Ok(CustomType::Unknown),
        }
    }
}
