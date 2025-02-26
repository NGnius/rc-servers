use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::tech_tree::*;

const PARAM_KEY: u8 = 210;

pub(super) fn tech_tree_layout_provider() -> SimpleFunc<183, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 104, // hashmap
            items: vec![
                TechTreeNode {
                    main_cube_id: 227205318, // default cube id
                    position_x: 0,
                    position_y: 0,
                    is_unlocked: true,
                    is_unlockable: true,
                    tech_points: 1,
                    neighbours: Vec::default(),
                }.as_transmissible_key_val(),
            ],
        }));
        Ok(params.into())
    })
}
