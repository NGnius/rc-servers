use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

const PARAM_KEY: u8 = 79;

pub(super) fn building_xp_config_provider() -> SimpleFunc<199, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 104, // hashtable
            items: vec![
                (Typed::Str("BuildXPSettings".into()), Typed::HashMap(vec![
                    (Typed::Str("buildModePeriodUserEarnXP".into()), Typed::Float(1.0)), // TODO what are the time units?
                    (Typed::Str("buildModePeriodUserInactivity".into()), Typed::Float(2.0)),
                ].into())),
            ],
        }));
        Ok(params.into())
    })
}
