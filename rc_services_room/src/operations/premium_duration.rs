use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const DAYS_PARAM_KEY: u8 = 8;
const HOURS_PARAM_KEY: u8 = 13;
const MINUTES_PARAM_KEY: u8 = 14;
const SECONDS_PARAM_KEY: u8 = 15;
const LIFETIME_PARAM_KEY: u8 = 150;

pub(super) fn premium_remaining_provider() -> SimpleFunc<15, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(DAYS_PARAM_KEY, Typed::Int(0));
        params.insert(HOURS_PARAM_KEY, Typed::Int(0));
        params.insert(MINUTES_PARAM_KEY, Typed::Int(0));
        params.insert(SECONDS_PARAM_KEY, Typed::Int(0));
        params.insert(LIFETIME_PARAM_KEY, Typed::Bool(false));
        Ok(params.into())
    })
}
