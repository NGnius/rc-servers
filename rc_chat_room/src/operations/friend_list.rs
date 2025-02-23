use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::friend::*;

const FRIENDS_PARAM_KEY: u8 = 5;
const AVATAR_PARAM_KEY: u8 = 76;

pub(super) fn friends_provider() -> SimpleFunc<4, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(FRIENDS_PARAM_KEY, Typed::Arr(Arr {
            ty: 99, // custom
            items: vec![Typed::Custom(vec![ // FIXME don't manually serialize
                0u8, // byte custom type
                0u8, 5u8, // short custom object size
                3u8, 0u8, 0u8, 0u8, 0u8, // content
            ].into())] }));
        params.insert(AVATAR_PARAM_KEY, Typed::Arr(Arr {
            ty: 104, // hashmap
            items: vec![AvatarInfo {
                name: "".to_string(),
                use_custom_avatar: false,
                avatar_id: 1,
            }.as_transmissible()],
        }));
        Ok(params.into())
    })
}
