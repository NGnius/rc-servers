use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 1;

pub(super) fn garage_colour_combo_provider() -> SimpleFunc<37, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Str("[[0,1,2,3]]".into())); // why tf is this a JSON in a string??? (list of lists of bytes)
        Ok(params.into())
    })
}
