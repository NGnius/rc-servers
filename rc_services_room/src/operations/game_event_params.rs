use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::custom_games::*;

const MAP_NAMES_PARAM_KEY: u8 = 78;
const VISIBILITY_PARAM_KEY: u8 = 66;
const MODE_PARAM_KEY: u8 = 136;
const AUTO_HEAL_PARAM_KEY: u8 = 37;
const REMAINING_TICKS_PARAM_KEY: u8 = 145;

pub(super) fn event_system_params_provider() -> SimpleFunc<24, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MAP_NAMES_PARAM_KEY, Typed::Arr(Arr {
            ty: 115, // str
            items: vec![
                Typed::Str("Assets/Scenes/Planet_Neptune/RC_Planet_Neptune_03_BA".into()),
                Typed::Str("Assets/Scenes/Planet_Earth/RC_Planet_Earth_01_BA".into()),
            ],
        }));
        params.insert(VISIBILITY_PARAM_KEY, Typed::Arr(Arr {
            ty: 105, // int
            items: vec![
                Typed::Int(GameMode::BattleArena as _),
                Typed::Int(GameMode::BattleArena as _),
            ],
        }));
        params.insert(MODE_PARAM_KEY, Typed::Arr(Arr {
            ty: 105, // int
            items: vec![
                Typed::Int(MapVisibility::Good as _),
                Typed::Int(MapVisibility::Bad as _),
            ],
        }));
        params.insert(AUTO_HEAL_PARAM_KEY, Typed::Arr(Arr {
            ty: 111, // bool
            items: vec![
                Typed::Bool(true.into()),
                Typed::Bool(false.into()),
            ],
        }));
        params.insert(REMAINING_TICKS_PARAM_KEY, Typed::Long(1_000_000));
        Ok(params.into())
    })
}
