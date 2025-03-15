use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};
use crate::persist::config::ConfigProvider;

const PARAM_KEY: u8 = 16;

pub(super) fn cube_inv_provider(cubes: &crate::persist::config::ConfigImpl) -> SimpleFunc<16, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    let cube_ids = <crate::persist::config::ConfigImpl as ConfigProvider<()>>::ids(cubes);
    SimpleFunc::new(move |params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Int, // int
            val_ty: TypePrefix::Int, // int
            items: cube_ids.iter().map(|id| (Typed::Int(*id as _), Typed::Int(1))).collect()}));
        Ok(params.into())
    })
}
