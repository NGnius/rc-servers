use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::custom_games::*;

const MODE_MAP_PARAM_KEY: u8 = 170;
const MAP_NAMES_PARAM_KEY: u8 = 178;

pub(super) fn allowed_maps_provider() -> SimpleFunc<146, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MODE_MAP_PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::ObjArr, // obj arr
            items: vec![
                (Typed::Str(GameMode::BattleArena.as_str().into()), Typed::ObjArr(vec![
                        Typed::Str("RC_Planet_Neptune_02_BA".into()),
                        Typed::Str("RC_Planet_Mars_03_BA".into()),
                        Typed::Str("RC_Planet_Mars_02_BA".into()),
                        Typed::Str("RC_Planet_Earth_02_BA".into()),
                        Typed::Str("RC_Planet_Earth_01_BA".into()),
                        Typed::Str("RC_Planet_Neptune_03_BA".into()),
                    ].into())
                ),
                (Typed::Str(GameMode::TeamDeathmatch.as_str().into()), Typed::ObjArr(vec![
                        Typed::Str("RC_Planet_Neptune_01_CTF".into()),
                        Typed::Str("RC_Planet_Mars_01_CTF".into()),
                    ].into())
                ),
            ],
        }));
        params.insert(MAP_NAMES_PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::Str, // str
            items: vec![
                (Typed::Str("RC_Planet_Neptune_02_BA".into()), Typed::Str("strCustomGameMapNameRC_Planet_Neptune_02_BA".into())),
                (Typed::Str("RC_Planet_Neptune_01_CTF".into()), Typed::Str("strCustomGameMapNameRC_Planet_Neptune_01_CTF".into())),
                (Typed::Str("RC_Planet_Mars_03_BA".into()), Typed::Str("strCustomGameMapNameRC_Planet_Mars_03_BA".into())),
                (Typed::Str("RC_Planet_Mars_02_BA".into()), Typed::Str("strCustomGameMapNameRC_Planet_Mars_02_BA".into())),
                (Typed::Str("RC_Planet_Mars_01_CTF".into()), Typed::Str("strCustomGameMapNameRC_Planet_Mars_01_CTF".into())),
                (Typed::Str("RC_Planet_Earth_02_BA".into()), Typed::Str("strCustomGameMapNameRC_Planet_Earth_02_BA".into())),
                (Typed::Str("RC_Planet_Earth_01_BA".into()), Typed::Str("strCustomGameMapNameRC_Planet_Earth_01_BA".into())),
                (Typed::Str("RC_Planet_Neptune_03_BA".into()), Typed::Str("strCustomGameMapNameRC_Planet_Neptune_03_BA".into())),
                //(Typed::Str("TestRobot".into()), Typed::Str("TestRobot".into())),
            ],
        }));
        Ok(params.into())
    })
}
