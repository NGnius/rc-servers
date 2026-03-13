use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const MODE_MAP_PARAM_KEY: u8 = 170;
const MAP_NAMES_PARAM_KEY: u8 = 178;

const ALL_MAPS: &[&str] = &[
    "RC_Planet_Mars_01_CTF", // og flat mars
    "RC_Planet_Mars_02_BA", // the one with the bridge in the middle
    "RC_Planet_Mars_03_BA", // tharsis rift without the rift
    "RC_Planet_Neptune_01_CTF", // og flat GJ1214b gliese lake without the lake
    "RC_Planet_Neptune_02_BA", // the one with the cave
    "RC_Planet_Neptune_03_BA", // spitzer dam
    "RC_Planet_Earth_01_BA", // birmingham power station
    "RC_Planet_Earth_02_BA", // vanguard
];

pub(super) fn allowed_maps_provider() -> SimpleFunc<146, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MODE_MAP_PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::ObjArr, // obj arr
            items: vec![
                (Typed::Str(oj_rc_core::data::game_mode::GameMode::BattleArena.as_str().into()), Typed::ObjArr(
                    ALL_MAPS.iter()
                        .map(|x| Typed::Str(x.into()))
                        .collect::<Vec<_>>()
                        .into()
                    )
                ),
                (Typed::Str(oj_rc_core::data::game_mode::GameMode::TeamDeathmatch.as_str().into()), Typed::ObjArr(
                    ALL_MAPS.iter()
                        .map(|x| Typed::Str(x.into()))
                        .collect::<Vec<_>>()
                        .into()
                    )
                ),
                (Typed::Str(oj_rc_core::data::game_mode::GameMode::Pit.as_str().into()), Typed::ObjArr(
                    ALL_MAPS.iter()
                        .map(|x| Typed::Str(x.into()))
                        .collect::<Vec<_>>()
                        .into()
                    )
                ),
                (Typed::Str(oj_rc_core::data::game_mode::GameMode::SuddenDeath.as_str().into()), Typed::ObjArr(
                    ALL_MAPS.iter()
                        .map(|x| Typed::Str(x.into()))
                        .collect::<Vec<_>>()
                        .into()
                    )
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
