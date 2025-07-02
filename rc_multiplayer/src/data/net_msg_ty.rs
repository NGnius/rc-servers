#[repr(i16)]
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    ClientMsg = 48,
    ServerMsg = 49,
    RobotMotion = 50,
}

impl MessageType {
    pub fn from_i16(num: i16) -> Option<Self> {
        match num {
            48 => Some(Self::ClientMsg),
            49 => Some(Self::ServerMsg),
            50 => Some(Self::RobotMotion),
            _ => None,
        }
    }
}
