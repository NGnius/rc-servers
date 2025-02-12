use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 61;

pub(super) fn power_bar_provider() -> SimpleFunc<51, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
                (Typed::Str("refillRatePerSecond".into()), Typed::Float(1.0)),
                (Typed::Str("powerForAllRobots".into()), Typed::Int(1_000 /* converted to u32 */)),
            ].into()
        ));
        Ok(params.into())
    })
}
