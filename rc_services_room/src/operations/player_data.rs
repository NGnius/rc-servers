use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::{garage_bay::*, weapon_list::ItemCategory};

const GARAGE_SLOT_KEY: u8 = 45;
const NAME_KEY: u8 = 42;
const CRF_ID_KEY: u8 = 35;
const ROBOT_CPU_KEY: u8 = 177;
const CONTROL_TYPE_KEY: u8 = 59;
const CONTROL_OPTIONS_KEY: u8 = 60;
const WEAPON_ORDER_KEY: u8 = 52;
const ITEM_CATEGORY_KEY: u8 = 56;

pub(super) fn player_data_provider() -> SimpleFunc<61, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(GARAGE_SLOT_KEY, Typed::Int(0));
        params.insert(NAME_KEY, Typed::Str("Reverse-engineer great success!".into()));
        params.insert(CRF_ID_KEY, Typed::Int(0));
        params.insert(ROBOT_CPU_KEY, Typed::Int(1));
        params.insert(CONTROL_TYPE_KEY, Typed::Int(ControlType::Camera as _));
        params.insert(CONTROL_OPTIONS_KEY, ControlOptions { vertical_strafing: false, sideways_driving: false, tracks_turn_on_spot: false, }.as_transmissible());
        params.insert(WEAPON_ORDER_KEY, Typed::Arr(Arr {
            ty: 105, // int
            items: vec![
                Typed::Int(0),
            ],
        }));
        params.insert(ITEM_CATEGORY_KEY, Typed::Arr(Arr {
            ty: 105, // int
            items: vec![
                Typed::Int(ItemCategory::Wheel.but_bigger()),
            ],
        }));
        Ok(params.into())
    })
}
