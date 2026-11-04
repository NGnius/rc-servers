use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 10;

const REUSE_SLOT_PARAM_KEY: u8 = 43; // int (optional); in
const ROBOT_DATA_PARAM_KEY: u8 = 46; // bytes; in
const COLOUR_DATA_PARAM_KEY: u8 = 33; // bytes; in

pub(super) struct GarageSlotPrebuiltAdder {
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
    weapon_order: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for GarageSlotPrebuiltAdder {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Bytes(robot_data)) = params.remove(&ROBOT_DATA_PARAM_KEY) {
            if let Some(Typed::Bytes(colour_data)) = params.remove(&COLOUR_DATA_PARAM_KEY) {
                let reuse_slot = if let Some(Typed::Int(slot)) = params.remove(&REUSE_SLOT_PARAM_KEY) {
                    Some(slot)
                } else {
                    None
                };
                let user_info = user.user()?;
                let new_slot = user_info.new_slot(reuse_slot).await?;
                let weapon_order = self.weapon_order.guess_weapons(&mut std::io::Cursor::new(&robot_data.vec));
                user_info.save_slot(
                    oj_rc_core::persist::user::VehicleData {
                        name: None,
                        slot: new_slot.slot_i,
                        robot_data: robot_data.vec,
                        colour_data: colour_data.vec,
                        weapon_order,
                        crf_id: None,
                        was_rated: None,
                    },
                    &self.cpu_counter,
                ).await?;
            }
        }
        Ok(params.into())
    }
}

pub(super) fn garage_slot_prebuilt_add_provider<C: Send + 'static>(parsers: &oj_rc_core::cubes::CubeParsers) -> SimpleOpImpl<C, crate::UserTy, GarageSlotPrebuiltAdder> {
    SimpleOpImpl::new(GarageSlotPrebuiltAdder {
        cpu_counter: parsers.cpu_counter(),
        weapon_order: parsers.weapon_order(),
    })
}
