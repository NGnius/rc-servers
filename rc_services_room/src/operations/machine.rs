use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::weapon_list::ItemCategory;
use crate::data::garage_bay::*;

const SLOT_PARAM_KEY: u8 = 45; // uint
const DATA_PARAM_KEY: u8 = 49; // byte arr
const CUBES_COUNT_PARAM_KEY: u8 = 51; // int
const WEAPON_ORDER_PARAM_KEY: u8 = 52; // int arr
const MOVEMENT_CATEGORIES_PARAM_KEY: u8 = 56; // int arr
const CONTROL_TYPE_PARAM_KEY: u8 = 59; // int
const CONTROL_OPTIONS_PARAM_KEY: u8 = 60; // bool arr
const MASTERY_LEVEL_PARAM_KEY: u8 = 18; // int

pub(super) fn garage_machine_provider() -> SimpleFunc<43, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(garage_slot) = params.get(&SLOT_PARAM_KEY) {
            log::debug!("Got machine request for slot {:?}", garage_slot);
        } else {
            params.insert(SLOT_PARAM_KEY, Typed::Int(0));
        }
        params.insert(DATA_PARAM_KEY, Typed::Bytes(vec![0u8, 0u8, 0u8, 0u8].into())); // first 4 bytes are i32 for length of rest of data
        params.insert(CUBES_COUNT_PARAM_KEY, Typed::Int(1));
        params.insert(WEAPON_ORDER_PARAM_KEY, Typed::IntArr(vec![0].into()));
        params.insert(MOVEMENT_CATEGORIES_PARAM_KEY, Typed::IntArr(vec![ItemCategory::Wheel.but_bigger()].into()));
        params.insert(CONTROL_TYPE_PARAM_KEY, Typed::Int(ControlType::Camera as _));
        params.insert(CONTROL_OPTIONS_PARAM_KEY, ControlOptions { vertical_strafing: false, sideways_driving: false, tracks_turn_on_spot: false, }.as_transmissible());
        params.insert(MASTERY_LEVEL_PARAM_KEY, Typed::Int(0));
        Ok(params.into())
    })
}
