use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::player_rank::*;

const PARAM_KEY: u8 = 80;

pub(super) fn rank_provider() -> SimpleFunc<80, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Int(0));
        Ok(params.into())
    })
}

const STATIC_PARAM_KEY: u8 = 1;

pub(super) fn rank_static_provider() -> SimpleFunc<126, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(STATIC_PARAM_KEY, PlayerRankStaticInfo {
            sub_rank_count: 5,
            sub_rank_thresholds: vec![
                0, 200, 500, 800, 1200,
                1600, 2000, 2500, 3000, 3500,
                4000, 6000, 8000, 10_000, 12_000,
                16_000, 20_000, 25_000, 30_000, 35_000,
                40_000, 50_000, 60_000, 70_000, 80_000,
            ],
        }.as_transmissible());
        Ok(params.into())
    })
}
