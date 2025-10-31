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

pub struct AuthError {
    pub message: String,
    pub code: crate::data::error_codes::AuthErrorCode,
}

#[async_trait::async_trait]
pub trait UserProvider<C> {
    async fn authenticate(&self, user: UserToken) -> Result<Box<dyn User<C> + Send + Sync>, AuthError>;

    async fn multiplayer_authenticate(&self, user: String) -> Result<Box<dyn User<C> + Send + Sync>, AuthError>;
}

#[async_trait::async_trait]
pub trait UserAuthenticator {
    async fn login(&self, info: UserInfo) -> Result<UserLoginInfo, AuthError>;
    async fn user_exists(&self, user: UserId) -> Result<bool, String>;
    async fn register(&self, info: RegistrationInfo) -> Result<i32, String>;
}

#[async_trait::async_trait]
pub trait User<C>: ChatUser + LobbyUser + MultiplayerUser + IntercomUser + CommonUser {
    async fn unlocked_parts(&self) -> Vec<u32>;
    async fn selected_garage(&self) -> (String, u32);
    async fn select_garage(&self, slot: i32) -> Result<(), i16>;
    async fn all_slots(&self) -> UserSlots<C>;
    async fn slot_by_id(&self, id: i32) -> Result<UserSlotData<C>, i16>;
    async fn save_slot(&self, vehicle: VehicleData, cpu_counter: &crate::cubes::CpuListParser) -> Result<(), i16>;
    async fn save_slot_order(&self, slots: Vec<i32>) -> Result<(), i16>;
    async fn new_slot(&self, reset_slot: Option<i32>) -> Result<NewSlotData<C>, i16>;
    async fn copy_slot(&self, slot: i32, into_slot: Option<i32>, append: &str) -> Result<(), i16>;
    async fn upgrade_slot(&self, increments: i32) -> Result<polariton::operation::Typed<C>, i16>;
    async fn save_slot_controls(&self, controls: ControlData) -> Result<(), i16>;
    async fn save_slot_customisations(&self, customs: CustomisationData) -> Result<(), i16>;
    async fn get_slot_customisations(&self, uuid: &str) -> Result<GetCustomisationData<C>, i16>;
    async fn set_slot_name(&self, slot: i32, name: String) -> Result<(), i16>;
    fn signup_date(&self) -> i64;
    async fn singleplayer_robots(&self, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, singleplayer_config: &crate::persist::config::SingleplayerConfig, cpu_counter: &crate::cubes::CpuListParser) -> Result<polariton::operation::Typed<C>, i16>;
    async fn prepare_factory_upload(&self, vehicle: VehicleUploadData) -> Result<oj_rc_factory::VehicleUploadInfo, i16>;
    async fn last_seen(&self) -> Result<u64, i16>;
    async fn get_avatar_info(&self) -> Result<GetAvatarInfo<C>, i16>;
    async fn set_avatar_info(&self, info: AvatarInfo) -> Result<(), i16>;
    fn current_game_event_setter(&self) -> Box<dyn GameEventSetter>;
}

#[async_trait::async_trait]
pub trait GameEventSetter: Send + Sync + 'static {
    async fn set_multiplayer(&self, event: CurrentGameEvent);
    async fn get_multiplayer(&self) -> Option<CurrentGameEvent>;
    async fn set_singleplayer(&self, event: CurrentGameEvent);
    async fn get_singleplayer(&self) -> Option<CurrentGameEvent>;
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
    pub(super) fn into_db(self) -> oj_rc_database::schema::garage::ControlType {
        match self {
            Self::Camera => oj_rc_database::schema::garage::ControlType::Camera,
            Self::Keyboard => oj_rc_database::schema::garage::ControlType::Keyboard,
            Self::Count => oj_rc_database::schema::garage::ControlType::Count,
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
pub trait ChatUser: CommonUser + IntercomUser {
    async fn subscribed_channels(&self) -> Result<polariton::operation::Typed<()>, i16>;
    async fn subscribed_channels_strings(&self) -> Result<Vec<String>, i16>;
    async fn add_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<polariton::operation::Typed<()>, i16>;
    async fn remove_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<(), i16>;
    //async fn has_pending_sanctions(&self) -> Result<bool, i16>;
    async fn get_sanctions(&self, username: String) -> Result<polariton::operation::Typed<()>, i16>;
    async fn set_sanction(&self, sanction: SetSanction) -> Result<(), i16>;
    async fn get_total_registered_users(&self) -> Result<u64, polariton_server::operations::SimpleOpError>;
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

#[async_trait::async_trait]
pub trait LobbyUser {
    fn user_id(&self) -> i32;
    async fn player_data(&self, cpu_counter: &crate::cubes::CpuListParser) -> Result<crate::data::player_data::PlayerData, polariton_server::operations::SimpleOpError>;
    async fn team_chooser(&self, game: &GameDescriptor) -> super::TeamChooser;
    async fn start_game(&self, game: GameDescriptor, players: Vec<PlayerLobbyDescriptor>, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, cpu_counter: &crate::cubes::CpuListParser, weapon_lister: &crate::cubes::WeaponListParser, team_chooser: &super::TeamChooser) -> Result<FakePlayers, polariton_server::operations::SimpleOpError>;
}

pub struct FakePlayers {
    pub players: Vec<(crate::data::player_data::PlayerData, crate::persist::config::ClientEmulator)>
}

pub struct CurrentGameEvent {
    pub map: String,
    pub visibility: crate::data::game_mode::MapVisibility,
    pub mode: crate::data::game_mode::GameMode,
    pub auto_heal: bool,
    pub start: i64, // seconds since Unix epoch
    pub end: i64, // seconds since Unix epoch
}

pub struct GameDescriptor {
    pub guid: String,
    pub map: String,
    pub mode: crate::data::game_mode::GameMode,
    pub visibility: crate::data::game_mode::MapVisibility,
    pub auto_heal: bool,
    pub is_ranked: bool,
    pub is_custom: bool,
    pub is_complete: bool,
}

pub struct PlayerLobbyDescriptor {
    pub user_id: i32,
    pub team: i32,
    pub public_id: String,
    pub display_name: String,
    pub group: Option<i32>,
}

#[derive(Clone)]
pub struct PlayerDescriptor {
    pub user_id: Option<i32>,
    pub player_id: u8,
    pub team: i32,
    pub group: Option<i32>,
    pub public_id: String,
    pub display_name: String,
    pub is_rewards_claimed: bool,
    pub mode: Option<crate::persist::config::ClientEmulator>,
}

#[derive(Debug)]
pub struct MultiplayerError {
    pub code: MultiplayerErrorCode,
    pub message: String,
}

impl core::fmt::Display for MultiplayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl core::error::Error for MultiplayerError {}

#[repr(u8)]
#[derive(Debug)]
pub enum MultiplayerErrorCode {
    HaxSpeed = 0,
    HaxException = 1,
    HaxTeleport = 2,
    HaxEacViolation = 6,
    HaxAfk = 7,
    HaxFirerange = 8,
    HaxFiredamage = 9,
    HaxFirerate = 10,
    HaxFireposition = 11,
    IncorrectGameGuid = 12,
    CustomString = 13,
    TimedOut = 14,
    GameEnded = 15,
}

#[async_trait::async_trait]
pub trait MultiplayerUser: CommonUser {
    fn user_id(&self) -> i32;
    fn user_name(&self) -> &'_ str;
    fn display_name(&self) -> &'_ str;
    async fn current_game(&self) -> Result<Option<GameDescriptor>, MultiplayerError>;
    async fn game_players(&self, guid: &str) -> Result<Vec<PlayerDescriptor>, MultiplayerError>;
    async fn complete_game(&self, guid: &str) -> Result<(), MultiplayerError>;
    async fn game_info(&self, guid: &str) -> Result<Option<GameDescriptor>, MultiplayerError>;
}

#[async_trait::async_trait]
pub trait IntercomUser {
    async fn save_custom_avatar(&self, image: Vec<u8>) -> Result<(), polariton_server::operations::SimpleOpError>;
    async fn webservice_listener(&self) -> Result<IntercomListener<super::intercom::IntercomWebServiceUserMessage>, polariton_server::operations::SimpleOpError>;
    async fn show_dev_message(&self, msg: super::intercom::IntercomDevMessage, to: Vec<String>);
}

pub struct IntercomListener<D: serde::de::DeserializeOwned> {
    pub(super) websocket: reqwest_websocket::WebSocket,
    pub(super) _d: std::marker::PhantomData<D>,
}

impl <D: serde::de::DeserializeOwned> IntercomListener<D> {
    pub async fn listen(self) -> impl futures::Stream<Item=Result<D, reqwest_websocket::Error>> + Unpin {
        use futures::StreamExt;
        self.websocket.map(|msg| msg.and_then(|msg| msg.json()))
    }
}

pub struct ResolvedVehicle {
    pub mastery: i32,
    pub tier: i32,
    pub robot_name: String,
    pub robot_map: Vec<u8>,
    pub robot_uuid: String,
    pub cpu: i32,
    pub weapon_order: Vec<i32>,
    pub colour_map: Vec<u8>,
    pub spawn_effect: String,
    pub death_effect: String,
    pub weapon_rank: std::collections::HashMap<i32, i32>,
}

#[async_trait::async_trait]
pub trait CommonUser: Send + Sync {
    async fn resolve_config_vehicle(&self, vehicle: &crate::persist::config::VehicleInfo, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, cpu_counter: &crate::cubes::CpuListParser) -> Result<ResolvedVehicle, polariton_server::operations::SimpleOpError>;
    fn public_id(&self) -> &'_ str;
    fn is_mod(&self) -> bool;
    fn is_admin(&self) -> bool;
    fn is_dev(&self) -> bool;
    fn is_banned(&self) -> bool;
}
