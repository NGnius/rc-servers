use std::collections::HashMap;

use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::cube_list::*;

const PARAM_KEY: u8 = 1;

pub(super) fn cube_list_provider() -> SimpleFunc<2, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 104, // hashtable
            items: vec![
                //(u32 in base16 aka hex, hashtable)
                (Typed::Str("DEADBEEF".into()), CubeInfo {
                    cpu: 1,
                    health: 1,
                    health_boost: 1.0,
                    grey_out_in_tutorial: false,
                    visibility: VisibilityMode::All,
                    indestructible: true,
                    category: 1,
                    placements: 63,
                    protonium: false,
                    unlocked_by_league: false,
                    league_unlock_index: 1,
                    stats: HashMap::default(),
                    description: "This is a very descriptive description".to_string(),
                    size: ItemTier::NoTier,
                    type_: ItemType::NoFunction,
                    ranking: 1,
                    cosmetic: false,
                    variant_of: "0".to_string(),
                    ignore_in_weapon_list: true,
                }.as_transmissible())
            ].into(),
        }));
        Ok(params.into())
    })
}
