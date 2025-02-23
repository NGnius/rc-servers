use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 54;

pub(super) fn garage_id_provider() -> SimpleFunc<177, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Str(format!("{}_{}", 12345, 54321).into()));
        Ok(params.into())
    })
}
