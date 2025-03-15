use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const SLOT_PARAM_KEY: u8 = 45; // uint
const DATA_PARAM_KEY: u8 = 49; // byte arr
const CUBES_COUNT_PARAM_KEY: u8 = 51; // int
const WEAPON_ORDER_PARAM_KEY: u8 = 52; // int arr
const MOVEMENT_CATEGORIES_PARAM_KEY: u8 = 56; // int arr
const CONTROL_TYPE_PARAM_KEY: u8 = 59; // int
const CONTROL_OPTIONS_PARAM_KEY: u8 = 60; // bool arr
const MASTERY_LEVEL_PARAM_KEY: u8 = 18; // int

pub(super) fn garage_machine_provider() -> SimpleFunc<43, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let mut params = params.to_dict();
        let lock = user.read().unwrap();
        let user_info = lock.user()?;
        if let Some(Typed::Int(garage_slot)) = params.get(&SLOT_PARAM_KEY) {
            log::debug!("Got machine request for slot {:?}", garage_slot);
            let machine = user_info.slot_by_id(*garage_slot)?;
            params.insert(DATA_PARAM_KEY, machine.data);
            params.insert(CUBES_COUNT_PARAM_KEY, machine.cube_count);
            params.insert(WEAPON_ORDER_PARAM_KEY, machine.weapon_order);
            params.insert(MOVEMENT_CATEGORIES_PARAM_KEY, machine.movement_categories);
            params.insert(CONTROL_TYPE_PARAM_KEY, machine.control_type);
            params.insert(CONTROL_OPTIONS_PARAM_KEY, machine.control_options);
            params.insert(MASTERY_LEVEL_PARAM_KEY, machine.mastery_level);
        } else {
            params.insert(SLOT_PARAM_KEY, Typed::Int(user_info.selected_garage_slot() as _));
        }
        Ok(params.into())
    })
}

const ERROR_PARAM_KEY: u8 = 47; // int
//const UUID_PARAM_KEY: u8 = 54; // str
const COMPRESSED_ROBOT_DATA_PARAM_KEY: u8 = 46; // byte arr
const COMPRESSED_COLOUR_DATA_PARAM_KEY: u8 = 33; // byte arr

const INVALID_ROBOT_ERR: i16 = 140;

pub(super) fn garage_machine_save_provider() -> SimpleFunc<41, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        log::debug!("machine save params: {:?}", params);
        let mut params = params.to_dict();
        if let Some(Typed::Int(slot_index)) = params.remove(&SLOT_PARAM_KEY) {
            if let Some(Typed::Bytes(robot_data)) = params.remove(&COMPRESSED_ROBOT_DATA_PARAM_KEY) {
                if let Some(Typed::Bytes(colour_data)) = params.remove(&COMPRESSED_COLOUR_DATA_PARAM_KEY) {
                    if let Some(Typed::Arr(weapon_order)) = params.remove(&WEAPON_ORDER_PARAM_KEY) {
                        let weapon_order_filtered: Vec<_> = weapon_order.items.into_iter().filter_map(|ty| if let Typed::Int(i) = ty { Some(i) } else { None }).collect();
                        let lock = user.read().unwrap();
                        let user_info = lock.user()?;
                        let vehicle_data = crate::persist::user::VehicleData {
                            id: slot_index,
                            robot_data: robot_data.vec,
                            colour_data: colour_data.vec,
                            weapon_order: weapon_order_filtered,
                        };
                        user_info.save_slot(vehicle_data)?;
                        let mut params_out = std::collections::HashMap::with_capacity(1);
                        params_out.insert(ERROR_PARAM_KEY, Typed::Int(0));
                        return Ok(params_out.into());
                    } else {
                        log::warn!("weapon order is not this type (or does not exist)");
                    }
                } else {
                    log::warn!("colour data is not this type (or does not exist)");
                }
            } else {
                log::warn!("robot data is not this type (or does not exist)");
            }
        } else {
            log::warn!("slot is not this type (or does not exist)");
        }
        Err(INVALID_ROBOT_ERR)
    })
}

