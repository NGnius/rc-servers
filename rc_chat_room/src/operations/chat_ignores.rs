use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

const PARAM_KEY: u8 = 15;

pub(super) fn ignores_provider() -> SimpleFunc<8, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: 115, // str
            items: vec![
                Typed::Str("Pluto".into()),
            ],
        }));
        Ok(params.into())
    })
}
