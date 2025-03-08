use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::friend::*;

const FRIENDS_PARAM_KEY: u8 = 5;
const AVATAR_PARAM_KEY: u8 = 76;

pub(super) fn friends_provider() -> SimpleFunc<4, crate::UserTy, impl (Fn(ParameterTable<crate::data::custom::CustomType>, &crate::UserTy) -> Result<ParameterTable<crate::data::custom::CustomType>, i16>) + Sync + Sync, crate::data::custom::CustomType> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(FRIENDS_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::Custom, // custom
            items: vec![Typed::Custom(crate::data::custom::CustomType::FriendInfo)] }));
        params.insert(AVATAR_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            items: vec![AvatarInfo {
                name: "".to_string(),
                use_custom_avatar: false,
                avatar_id: 1,
            }.as_transmissible()],
        }));
        Ok(params.into())
    })
}
