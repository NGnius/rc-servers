use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Arr, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::custom_games::*;

const MAP_NAMES_PARAM_KEY: u8 = 78;
const VISIBILITY_PARAM_KEY: u8 = 66;
const MODE_PARAM_KEY: u8 = 136;
const AUTO_HEAL_PARAM_KEY: u8 = 37;
const REMAINING_TICKS_PARAM_KEY: u8 = 145;

/* Valid map names
RC_Planet_Mars_02_BA
RC_Planet_Mars_03_BA
RC_Planet_Neptune_02_BA
RC_Planet_Neptune_03_BA
RC_Planet_Earth_01_BA
RC_Planet_Earth_02_BA
RC_Planet_Mars_01_CTF
RC_Planet_Neptune_01_CTF
*/

pub(super) fn event_system_params_provider() -> SimpleFunc<24, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MAP_NAMES_PARAM_KEY, Typed::Arr(Arr {
            ty: TypePrefix::Str, // str
            items: vec![
                Typed::Str("RC_Planet_Neptune_03_BA".into()),
                Typed::Str("RC_Planet_Mars_02_BA".into()),
            ],
        }));
        params.insert(VISIBILITY_PARAM_KEY, Typed::Arr(Arr {
            ty: TypePrefix::Int, // int
            items: vec![
                Typed::Int(MapVisibility::Good as _),
                Typed::Int(MapVisibility::Bad as _),
            ],
        }));
        params.insert(MODE_PARAM_KEY, Typed::Arr(Arr {
            ty: TypePrefix::Int, // int
            items: vec![
                Typed::Int(GameMode::SinglePlayer as _),
                Typed::Int(GameMode::BattleArena as _),
            ],
        }));
        params.insert(AUTO_HEAL_PARAM_KEY, Typed::Arr(Arr {
            ty: TypePrefix::Bool, // bool
            items: vec![
                Typed::Bool(true.into()),
                Typed::Bool(false.into()),
            ],
        }));
        params.insert(REMAINING_TICKS_PARAM_KEY, Typed::Long(24 * 60 * 60 * 1_000_000 * 10 /* 24 hours in ticks (100ns units) */));
        Ok(params.into())
    })
}
