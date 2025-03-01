use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const TEST_PARAM_KEY: u8 = 166;
const TEST_GROUP_PARAM_KEY: u8 = 167;

pub(super) fn test_group_provider() -> SimpleFunc<55, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(TEST_PARAM_KEY, Typed::Str("RE_AB_test".into()));
        params.insert(TEST_GROUP_PARAM_KEY, Typed::Str("RE_AB_test_group".into()));
        Ok(params.into())
    })
}
