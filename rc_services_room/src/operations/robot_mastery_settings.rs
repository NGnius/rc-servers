use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 193;

pub(super) fn robot_mastery_settings_provider() -> SimpleFunc<73, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
                (Typed::Str("RobitsRewardForCRFRobotCreator".into()), Typed::Int(1_000)),
            ].into()));
        Ok(params.into())
    })
}
