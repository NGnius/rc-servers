use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::special_item::*;

const PARAM_KEY: u8 = 1;

pub(super) fn special_item_list_provider() -> SimpleFunc<6, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 104, // hashtable
            items: vec![
                //(u32 in base16 aka hex, hashtable)
                (Typed::Str("DEADBEEF".into()), SpecialItem {
                    name: "cool".to_string(),
                    sprite: "chair".to_string(),
                    size: 1,
                }.as_transmissible())
            ].into(),
        }));
        Ok(params.into())
    })
}
