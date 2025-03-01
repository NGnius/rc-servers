use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::player_rank::*;

const PARAM_KEY: u8 = 80;

pub(super) fn rank_provider() -> SimpleFunc<80, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Int(3));
        Ok(params.into())
    })
}

const STATIC_PARAM_KEY: u8 = 1;

pub(super) fn rank_static_provider() -> SimpleFunc<126, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(STATIC_PARAM_KEY, PlayerRankStaticInfo {
            sub_rank_thresholds: vec![1, 2, 3, 4, 5],
        }.as_transmissible());
        Ok(params.into())
    })
}
