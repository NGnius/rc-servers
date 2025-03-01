use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::custom_games::*;

const PARAM_KEY: u8 = 168;

pub(super) fn team_setup_provider() -> SimpleFunc<162, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 105, // int
            items: vec![
                (Typed::Str(GameMode::BattleArena.as_str().into()), Typed::Int(10)),
                (Typed::Str(GameMode::SuddenDeath.as_str().into()), Typed::Int(10)),
                (Typed::Str(GameMode::Pit.as_str().into()), Typed::Int(10)),
                (Typed::Str(GameMode::TestMode.as_str().into()), Typed::Int(10)),
                (Typed::Str(GameMode::SinglePlayer.as_str().into()), Typed::Int(1)),
                (Typed::Str(GameMode::TeamDeathmatch.as_str().into()), Typed::Int(10)),
                (Typed::Str(GameMode::Campaign.as_str().into()), Typed::Int(6)),
            ] }));
        Ok(params.into())
    })
}
