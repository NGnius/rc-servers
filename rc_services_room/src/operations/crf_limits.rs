use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 100;

pub(super) fn robot_shop_submission_infos_provider() -> SimpleFunc<95, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
                (Typed::Str("submissionCount".into()), Typed::Int(0)),
                (Typed::Str("maxSubmissions".into()), Typed::Int(10)),
            ].into()));
        Ok(params.into())
    })
}
