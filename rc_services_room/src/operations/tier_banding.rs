use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

const PARAM_KEY: u8 = 1;

pub(super) fn tiers_banding_provider() -> SimpleFunc<7, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 42, // obj
            items: vec![
                (Typed::Str("tiersbands".into()), Typed::IntArr(vec![
                    1
                ].into())),
                (Typed::Str("maximumRobotRankingARobotCanObtain".into()), Typed::Int(1)),
            ].into(),
        }));
        Ok(params.into())
    })
}
