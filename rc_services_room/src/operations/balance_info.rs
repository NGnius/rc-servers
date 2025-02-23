use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const FREE_BALANCE_PARAM_KEY: u8 = 74;
const PAID_BALANCE_PARAM_KEY: u8 = 87;

pub(super) fn balance_wallet_provider() -> SimpleFunc<66, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(FREE_BALANCE_PARAM_KEY, Typed::Long(31337_000));
        params.insert(PAID_BALANCE_PARAM_KEY, Typed::Long(1));
        Ok(params.into())
    })
}
