use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 60;
//const USER_PARAM_KEY: u8 = 1; // str (username)

pub(super) fn pending_battle_rewards_provider<C: Send + Sync>() -> SimpleFunc<54, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Bool(false));
        Ok(params.into())
    })
}
