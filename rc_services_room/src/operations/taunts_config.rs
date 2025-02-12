use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::taunts_config::*;

const PARAM_KEY: u8 = 195;

pub(super) fn taunts_config_provider() -> SimpleFunc<164, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(polariton::operation::Dict {
            key_ty: 115,
            val_ty: 42,
            items: vec![
                (Typed::Str("taunts".into()), TauntsData {
                    taunts: vec![
                        TauntData {
                            group_name: "totally_real_group_name".to_string(),
                            assets: AssetData {
                                idle_effect: "tbd".to_string(),
                                active_effect: "something".to_string(),
                                sound_effect: "3rd thing here".to_string(),
                            },
                            animation_offset_x: 0.0,
                            animation_offset_y: 0.0,
                            animation_offset_z: 0.0,
                            cubes: vec![
                                CubeData {
                                    cube_id: 1,
                                    position_x: 0,
                                    position_y: 0,
                                    position_z: 0,
                                    rotation: 0,
                                }
                            ]
                        }
                    ]
                }.as_transmissible())
            ]
        }));
        Ok(params.into())
    })
}
