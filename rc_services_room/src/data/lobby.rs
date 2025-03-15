#[repr(i8)]
#[derive(Debug)]
pub enum LobbyType {
    None = -1,
    CustomGame = 1,
    QuickPlay = 2,
    Solo = 3
}

impl LobbyType {
    pub fn from_int(i: i32) -> Result<Self, i16> {
        match i {
            -1 => Ok(Self::None),
            1 => Ok(Self::CustomGame),
            2 => Ok(Self::QuickPlay),
            3 => Ok(Self::Solo),
            _ => Err(super::error_codes::WebServicesError::UnexpectedError as _),
        }
    }
}
