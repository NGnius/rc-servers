use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const PARAM_KEY: u8 = 1;

pub(super) fn garage_upgrades_provider() -> SimpleFunc<1, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("cpuIncreaseCost".into()), Typed::Dict(Dict {
                key_ty: TypePrefix::Int, // int
                val_ty: TypePrefix::Int, // int
                items: vec![
                    // (CPU limit, upgrade cost)
                    (Typed::Int(100), Typed::Int(100)),
                    (Typed::Int(200), Typed::Int(200)),
                    (Typed::Int(1_000), Typed::Int(1_000)),
                    (Typed::Int(2_000), Typed::Int(2_000)), // max regular bot CPU
                    (Typed::Int(10_000), Typed::Int(10_000)), // max mega bot cpu
                ],
            }))
        ].into()));
        Ok(params.into())
    })
}
