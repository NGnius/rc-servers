use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

//const BAY_ID_KEY: u8 = 54; // in
const BAY_SKIN_KEY: u8 = 234;
const SPAWN_EFFECT_KEY: u8 = 235;
const DEATH_EFFECT_KEY: u8 = 236;

pub(super) fn bay_customisations_provider() -> SimpleFunc<218, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(BAY_SKIN_KEY, Typed::Str("RC_MothershipSkin_Neptune_01".into()));
        params.insert(SPAWN_EFFECT_KEY, Typed::Str("RE_todo_spawn_effect".into()));
        params.insert(DEATH_EFFECT_KEY, Typed::Str("RE_todo_death_effect".into()));
        Ok(params.into())
    })
}
