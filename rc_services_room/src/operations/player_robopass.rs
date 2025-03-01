use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::player_robopass_season::*;

const PARAM_KEY: u8 = 237;

pub(super) fn player_robopass_season_provider() -> SimpleFunc<178, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, PlayerRoboPassSeasonInfo {
            delta_xp_to_show: 42,
            grade: 1,
            has_deluxe: true,
            progress_in_grade: 0.5,
            xp_from_start: 12345,
        }.as_transmissible());
        Ok(params.into())
    })
}
