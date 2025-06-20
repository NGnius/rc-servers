#[repr(i16)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum WebServicesError {
    None = 0,
    CPUTooHigh = 1,
    CPUTooLow = 2,
    RobotTierNotAllowed = 3,
    CubeIDNotAllowed = 4,
    CubeTypeNotAllowed = 5,
    DatabaseError = 8,
    UnexpectedError = 9,
    WrongNumberOfAuthParams = 10,
    Banned = 11,
    EACValidationFailed = 12,
    NotSteamUser = 13,
    PromotionDoesntExist = 14,
    NotEnoughMoney = 17,
    MaxGarageSlots = 18,
    PlatformFeatureNotAvailable = 19,
    UserDoesNotHaveAllCubeTypes = 20,
    ReplaceDailyQuestLimit = 21,
    MaintenanceModeError = 125,
    RobotShopMaintenanceMode = 126,
    InvalidRobot = 140,
    ExpiredRobot = 144,
    RobotHasSanction = 145,
    CustomisationNotOwned = 146,
    ItemShopBundleExpired = 147,
    UserNotFound = 200,
    InvalidUsernameFormat = 201,
    UsernameTooLong = 202,
    InvalidUsername = 203,
    TencentValidationFail = 204,
    UsernameAlreadyTaken = 205,
    UsernameTooShort = 206,
    SaleEnded = 207
}

#[repr(i16)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum ChatErrorCodes {
    None = 0,
    UnexpectedError = 1,
    Flood = 2,
    Muted = 3,
    NotOnline = 4,
    DoesNotExist = 5,
    NoConnection = 6,
    ModeratorsOnly = 7,
    AdminsOnly = 8,
    SanctionAlreadyExists = 9,
    AlreadyWarned = 10,
    NoSanctionExists = 11,
    MaintenanceMode = 12,
    ChannelExists = 13,
    IncorrectPassword = 14,
    ChannelNotExists = 15,
    PasswordRequired = 16,
    ChannelExpired = 17,
}

#[repr(i16)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum SingleplayerErrorCode {
    None = 0,
    DatabaseError = 1,
    UnexpectedError = 2,
    WrongNumberOfAuthParams = 3,
    MaintenanceMode = 4,
    DuplicateLogin = 5,
}

#[repr(i16)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum LobbyReasonCode {
    UnexpectedError = -1,
    Ok = 0,
    MaintenanceMode = 1,
    RobotValidationError = 2,
    LoggedInOtherLocation = 3,
    GroupFailedChecks = 4,
    ConnectionTestFailed = 5,
    WrongGameModeForParty = 6,
    BrawlConnectionTestFailed = 7,
    PartyNotAllowed = 8,
    NoSuitableLobbyFound = 9,
    EventSystemExpired = 10
}

impl LobbyReasonCode {
    pub(crate) fn from_service_error(err: i16) -> Self {
        match err {
            0 /* None */ => Self::Ok,
            125 => Self::MaintenanceMode,
            140 => Self::RobotValidationError,
            _ => Self::UnexpectedError,
        }
    }
}
