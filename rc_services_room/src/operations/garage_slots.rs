use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict, Arr};

use crate::data::garage_bay::*;

const SLOTS_PARAM_KEY: u8 = 44;
const SELECTED_SLOT_PARAM_KEY: u8 = 43;
const SLOT_ORDER_PARAM_KEY: u8 = 58;

pub(super) fn garage_slot_provider() -> SimpleFunc<40, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(SLOTS_PARAM_KEY, Typed::Dict(Dict {
            key_ty: 105, // int
            val_ty: 104, // hashmap
            items: vec![
                (Typed::Int(0), GarageSlotInfo {
                    name: "Reverse-engineer great success!".to_owned(),
                    cubes: 1,
                    crf_id: 0,
                    was_rated: false,
                    movement_categories: vec![MovementCategory::Wheel],
                    uuid: (2,4),
                    thumbnail_version: 0,
                    total_robot_cpu: 1,
                    total_cosmetic_cpu: 0,
                    total_robot_ranking: 1,
                    bay_cpu: 2_000,
                    tutorial_robot: false,
                    starter_robot_index: -1,
                    control_type: 0,
                    control_options: vec![false, false],
                    mastery_level: 1,
                    bay_skin_id: "".to_owned(), // TODO
                    weapon_order: vec![0],
                }.as_transmissible())
            ],
        }));
        params.insert(SELECTED_SLOT_PARAM_KEY, Typed::Int(0));
        params.insert(SLOT_ORDER_PARAM_KEY, Typed::Arr(Arr {
            ty: 105, // int
            items: vec![Typed::Int(0)],
        }));
        Ok(params.into())
    })
}
