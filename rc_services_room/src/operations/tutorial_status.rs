use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const IN_PROGRESS_PARAM_KEY: u8 = 140;
const COMPLETED_PARAM_KEY: u8 = 141;
const SKIPPED_PARAM_KEY: u8 = 142;

pub(super) fn tutorial_info_provider() -> SimpleFunc<122, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(IN_PROGRESS_PARAM_KEY, Typed::Bool(false));
        params.insert(COMPLETED_PARAM_KEY, Typed::Bool(true));
        params.insert(SKIPPED_PARAM_KEY, Typed::Bool(true));
        Ok(params.into())
    })
}
