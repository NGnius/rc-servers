#[allow(dead_code)]
#[derive(Debug)]
pub struct UserToken {
    pub uuid: String,
    pub token: String,
    pub refresh_token: String,
}

pub trait UserProvider<C> {
    fn authenticate(&self, user: UserToken) -> Result<Box<dyn User<C> + Send + Sync>, String>;
}

pub trait User<C> {
    fn token(&self) -> &'_ super::UserToken;
    fn is_mod(&self) -> bool;
    fn is_admin(&self) -> bool;
    fn is_dev(&self) -> bool;
    fn unlocked_parts(&self) -> Vec<u32>;
    fn selected_garage_uuid(&self) -> String;
    fn selected_garage_slot(&self) -> u32;
    fn all_slots_by_id(&self) -> UserSlots<C>;
    fn slot_by_id(&self, id: i32) -> Result<UserSlotData<C>, i16>;
    fn save_slot(&self, vehicle: VehicleData) -> Result<(), i16>;
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
}

pub struct VehicleData {
    pub id: i32,
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub weapon_order: Vec<i32>,
}
