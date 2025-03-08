#[derive(Debug, Clone)]
pub enum CustomType {
    FriendInfo, // TODO actually serialise
}

pub struct CustomTypeSerdes;

impl polariton::serdes::CustomSerdes<CustomType> for CustomTypeSerdes {
    fn dump(_c: &CustomType, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let payload = vec![ // FIXME don't manually serialize
            0u8, // byte custom type
            0u8, 5u8, // short custom object size
            3u8, 0u8, 0u8, 0u8, 0u8, // content
        ];
        w.write(&payload)
    }

    fn parse(_r: &mut dyn std::io::Read) -> std::io::Result<CustomType> {
        Ok(CustomType::FriendInfo)
    }
}
