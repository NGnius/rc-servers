use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::custom_games::*;

const PARAM_KEY: u8 = 168;
//const INVITE_PARAM_KEY: u8 = 189; // hashtable (refer to C# CheckIfHasBeenInvitedToCustomGameSessionRequest)

pub(super) fn pending_invite_provider() -> SimpleFunc<0, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Int(CustomGameInviteCode::NoInvite as _));
        Ok(params.into())
    })
}
