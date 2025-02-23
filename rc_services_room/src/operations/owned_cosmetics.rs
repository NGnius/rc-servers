use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

const PARAM_KEY: u8 = 50;

pub(super) fn owned_cosmetics_provider() -> SimpleFunc<23, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: 115, // str
            items: vec![Typed::Str("1".into())],
        }));
        Ok(params.into())
    })
}

pub(super) fn selected_cosmetics_provider() -> SimpleFunc<21, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: 115, // str
            items: vec![Typed::Str("1".into())],
        }));
        Ok(params.into())
    })
}
