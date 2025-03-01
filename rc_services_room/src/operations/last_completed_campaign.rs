use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

// const CAMPAIGN_ID_PARAM_KEY: u8 = 22; // str
// const DIFFICULTY_PARAM_KEY: u8 = 22; // int
const AVAILABLE_PARAM_KEY: u8 = 89; // bool

pub(super) fn completed_campaign_provider() -> SimpleFunc<77, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(AVAILABLE_PARAM_KEY, Typed::Bool(false.into()));
        Ok(params.into())
    })
}
