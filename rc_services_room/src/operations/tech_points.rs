use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const CURRENT_PARAM_KEY: u8 = 214;

pub(super) fn tech_points_provider() -> SimpleFunc<187, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(CURRENT_PARAM_KEY, Typed::Int(1337));
        Ok(params.into())
    })
}

const UNCLAIMED_PARAM_KEY: u8 = 212;

pub(super) fn tech_points_awards_provider() -> SimpleFunc<185, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(UNCLAIMED_PARAM_KEY, Typed::Int(0));
        Ok(params.into())
    })
}
