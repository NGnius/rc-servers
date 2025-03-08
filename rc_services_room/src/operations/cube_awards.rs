use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Arr, ParameterTable, Typed}, serdes::TypePrefix};

const PARAM_KEY: u8 = 216;

pub(super) fn cube_awards_provider() -> SimpleFunc<206, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: TypePrefix::Str, // str
            items: Vec::default(),
        }));
        Ok(params.into())
    })
}
