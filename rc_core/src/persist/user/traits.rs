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

pub struct UserLoginInfo {
    pub response: libfj::robocraft::AuthenticationResponseInfo,
    pub is_new: bool,
}

#[async_trait::async_trait]
pub trait UserProvider<C> {
    async fn authenticate(&self, user: UserToken, ext: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>) -> Result<Box<dyn User<C> + Send + Sync>, String>;
}

#[async_trait::async_trait]
pub trait UserAuthenticator {
    async fn login(&self, info: UserInfo) -> Result<UserLoginInfo, String>;
}

#[async_trait::async_trait]
pub trait User<C> {
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
    fn signup_date(&self) -> i64;
    async fn singleplayer_robots(&self) -> Result<polariton::operation::Typed<C>, i16>;
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
}

pub struct VehicleData {
    pub slot: i32,
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub weapon_order: Vec<i32>,
}
