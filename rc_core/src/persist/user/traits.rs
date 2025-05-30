#[allow(dead_code)]
#[derive(Debug)]
pub struct UserToken {
    pub uuid: String,
    pub token: String,
    pub refresh_token: String,
}

pub struct UserInfo {
    pub payload: libfj::robocraft::TokenPayload,
    pub extra: ExtraUserInfo,
}

pub enum ExtraUserInfo {
    Steam {
        id: u64,
    },
    Username {
        password: String,
    },
    Email {
        password: String,
    }
}

pub enum UserId {
    SteamId(u64),
    Email(String),
    Username(String),
}

pub struct UserLoginInfo {
    pub response: libfj::robocraft::AuthenticationResponseInfo,
    pub is_new: bool,
}

pub struct RegistrationInfo {
    pub display_name: String,
    pub password: String,
    pub email: Option<String>,
    pub steam_id: Option<u64>,
}

#[async_trait::async_trait]
pub trait UserProvider<C> {
    async fn authenticate(&self, user: UserToken, ext: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>) -> Result<Box<dyn User<C> + Send + Sync>, String>;
}

#[async_trait::async_trait]
pub trait UserAuthenticator {
    async fn login(&self, info: UserInfo) -> Result<UserLoginInfo, String>;
    async fn user_exists(&self, user: UserId) -> Result<bool, String>;
    async fn register(&self, info: RegistrationInfo) -> Result<u32, String>;
}

#[async_trait::async_trait]
pub trait User<C>: ChatUser {
    fn ext(&self, ty: std::any::TypeId) -> Option<&'_ (dyn std::any::Any + Send + Sync + 'static)>;
    fn token(&self) -> &'_ super::UserToken;
    fn is_mod(&self) -> bool;
    fn is_admin(&self) -> bool;
    fn is_dev(&self) -> bool;
    async fn unlocked_parts(&self) -> Vec<u32>;
    async fn selected_garage(&self) -> (String, u32);
    async fn select_garage(&self, slot: i32) -> Result<(), i16>;
    async fn all_slots(&self) -> UserSlots<C>;
    async fn slot_by_id(&self, id: i32) -> Result<UserSlotData<C>, i16>;
    async fn save_slot(&self, vehicle: VehicleData) -> Result<(), i16>;
    async fn save_slot_order(&self, slots: Vec<i32>) -> Result<(), i16>;
    async fn new_slot(&self, reset_slot: Option<i32>) -> Result<NewSlotData<C>, i16>;
    async fn upgrade_slot(&self, increments: i32) -> Result<polariton::operation::Typed<C>, i16>;
    async fn save_slot_controls(&self, controls: ControlData) -> Result<(), i16>;
    async fn save_slot_customisations(&self, customs: CustomisationData) -> Result<(), i16>;
    async fn get_slot_customisations(&self, uuid: &str) -> Result<GetCustomisationData<C>, i16>;
    fn signup_date(&self) -> i64;
    async fn singleplayer_robots(&self) -> Result<polariton::operation::Typed<C>, i16>;
    async fn prepare_factory_upload(&self, vehicle: VehicleUploadData) -> Result<rc_factory::VehicleUploadInfo, i16>;
    async fn last_seen(&self) -> Result<u64, i16>;
    async fn get_avatar_info(&self) -> Result<GetAvatarInfo<C>, i16>;
    async fn set_avatar_info(&self, info: AvatarInfo) -> Result<(), i16>;
}

pub struct UserSlots<C> {
    pub slot_info: polariton::operation::Typed<C>,
    pub slot_order: polariton::operation::Typed<C>,
}

pub struct UserSlotData<C> {
    pub data: polariton::operation::Typed<C>,
    pub colour_data: polariton::operation::Typed<C>,
    pub cube_count: polariton::operation::Typed<C>,
    pub weapon_order: polariton::operation::Typed<C>,
    pub movement_categories: polariton::operation::Typed<C>,
    pub control_type: polariton::operation::Typed<C>,
    pub control_options: polariton::operation::Typed<C>,
    pub mastery_level: polariton::operation::Typed<C>,
    pub robot_rank: polariton::operation::Typed<C>,
    pub cpu: polariton::operation::Typed<C>,
    pub cosmetic_cpu: polariton::operation::Typed<C>,
    pub uuid: polariton::operation::Typed<C>,
}

pub struct NewSlotData<C> {
    pub name: polariton::operation::Typed<C>,
    pub uuid_0: polariton::operation::Typed<C>,
    pub uuid_1: polariton::operation::Typed<C>,
    pub slot: polariton::operation::Typed<C>,
    pub bay_cpu: polariton::operation::Typed<C>,
    pub mastery_level: polariton::operation::Typed<C>,
    pub slot_i: i32,
}

pub struct VehicleData {
    pub name: Option<String>,
    pub slot: i32,
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub weapon_order: Vec<i32>,
    pub crf_id: Option<i32>,
}

pub struct VehicleUploadData {
    pub version: String,
    pub slot: i32,
    pub name: String,
    pub description: String,
    pub thumbnail: Vec<u8>,
}

pub struct ControlData {
    pub slot: i32,
    pub control_ty: ControlType,
    pub vertical_strafing: bool,
    pub sideways_driving: bool,
    pub tracks_turn_on_spot: bool,
}

pub enum ControlType {
    Camera = 0,
    Keyboard = 1,
    Count = 2,
}

impl ControlType {
    pub fn from_i32(i: i32) -> Result<Self, i16> {
        match i {
            0 => Ok(Self::Camera),
            1 => Ok(Self::Keyboard),
            2 => Ok(Self::Count),
            _ => Err(crate::data::error_codes::WebServicesError::UnexpectedError as i16),
        }
    }

    #[inline]
    pub(super) fn into_db(self) -> rc_database::schema::garage::ControlType {
        match self {
            Self::Camera => rc_database::schema::garage::ControlType::Camera,
            Self::Keyboard => rc_database::schema::garage::ControlType::Keyboard,
            Self::Count => rc_database::schema::garage::ControlType::Count,
        }
    }
}

pub struct CustomisationData {
    pub uuid: String,
    pub bay: String,
    pub spawn: String,
    pub death: String,
}

pub struct GetCustomisationData<C> {
    pub bay: polariton::operation::Typed<C>,
    pub spawn: polariton::operation::Typed<C>,
    pub death: polariton::operation::Typed<C>,
}

pub struct GetAvatarInfo<C> {
    pub avatar_id: polariton::operation::Typed<C>,
    pub use_custom: polariton::operation::Typed<C>,
}

pub struct AvatarInfo {
    pub avatar_id: i32,
    pub use_custom: bool,
}

#[async_trait::async_trait]
pub trait ChatUser {
    async fn subscribed_channels(&self) -> Result<polariton::operation::Typed<()>, i16>;
    async fn subscribed_channels_strings(&self) -> Result<Vec<String>, i16>;
    async fn add_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<polariton::operation::Typed<()>, i16>;
    async fn remove_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<(), i16>;
    //async fn has_pending_sanctions(&self) -> Result<bool, i16>;
    async fn get_sanctions(&self, username: String) -> Result<polariton::operation::Typed<()>, i16>;
    async fn set_sanction(&self, sanction: SetSanction) -> Result<(), i16>;
}

pub struct SetSanction {
    pub is_adding: bool, // if false, it's modifying
    pub type_: SanctionType,
    pub duration: i32,
    pub reason: String,
    pub username: String,
}

pub enum SanctionType {
    Warn = 0,
    Mute = 1,
    Ban = 2,
    Note = 3,
    Kick = 4,
}

impl SanctionType {
    pub fn from_i32(i: i32) -> Result<Self, i16> {
        match i {
            0 => Ok(Self::Warn),
            1 => Ok(Self::Mute),
            2 => Ok(Self::Ban),
            3 => Ok(Self::Note),
            4 => Ok(Self::Kick),
            _ => Err(crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16),
        }
    }
}
