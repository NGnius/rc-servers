use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 96;

pub(super) fn robot_shop_user_earnings_provider() -> SimpleFunc<88, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
                (Typed::Str("buyCount".into()), Typed::Int(0)),
                (Typed::Str("earnings".into()), Typed::Int(0)),
            ].into()));
        Ok(params.into())
    })
}
