use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::friend::*;

const INVITER_NAME_PARAM_KEY: u8 = 19;
const INVITER_DISPLAY_NAME_PARAM_KEY: u8 = 75;
const INVITER_CUSTOM_AVATAR_NAME_PARAM_KEY: u8 = 13;
const INVITER_AVATAR_ID_NAME_PARAM_KEY: u8 = 14;

pub(super) fn platoon_pending_provider<C: Send + Sync>() -> SimpleFunc<19, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(INVITER_NAME_PARAM_KEY, Typed::Str("RE_platoon_inviter".into())));
        params.insert(INVITER_DISPLAY_NAME_PARAM_KEY, Typed::Str("RE_platoon_inviter_display".into())));
        params.insert(INVITER_CUSTOM_AVATAR_NAME_PARAM_KEY, Typed::Bool(false.into())));
        params.insert(INVITER_AVATAR_ID_NAME_PARAM_KEY, Typed::Int(1)));
        Ok(params.into())
    })
}
