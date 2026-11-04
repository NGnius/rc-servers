use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

use oj_rc_core::data::robot_data::*;

const PARAM_KEY: u8 = 1;

pub(super) fn garage_robot_data_provider() -> SimpleFunc<4, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        let default_cube_id_bytes = 227205318u32.to_le_bytes();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashmap
            items: vec![
                (Typed::Str(format!("{}_{}", 12345, 54321).into()), PrebuiltRobotInfo {
                    name: "Prebuilt Front".to_owned(),
                    class: "History 101".to_owned(),
                    category: "Prebuilt Category".to_owned(),
                    robot_data: vec![1u8, 0u8, 0u8, 0u8, default_cube_id_bytes[0], default_cube_id_bytes[1], default_cube_id_bytes[2], default_cube_id_bytes[3], 0u8, 0u8, 0u8, 0u8],
                    colour_data: vec![1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8],
                }.as_transmissible()),
            ]
        }));
        Ok(params.into())
    })
}
