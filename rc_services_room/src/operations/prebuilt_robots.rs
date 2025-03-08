use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::robot_data::*;

const PARAM_KEY: u8 = 1;

pub(super) fn garage_robot_data_provider() -> SimpleFunc<4, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashmap
            items: vec![
                (Typed::Str(format!("{}_{}", 12345, 54321).into()), PrebuiltRobotInfo {
                    name: "Reverse-engineer great success! prebuilt_name".to_owned(),
                    class: "RE_robot_class0".to_owned(),
                    category: "RE_robot_category0".to_owned(),
                    robot_data: vec![0u8, 0u8, 0u8, 0u8], // first 4 bytes are i32 for the cube count (we want it to be 0)
                    colour_data: vec![0u8, 0u8, 0u8, 0u8],
                }.as_transmissible()),
            ]
        }));
        Ok(params.into())
    })
}
