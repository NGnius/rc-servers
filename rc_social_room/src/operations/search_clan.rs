use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan::*;

// params in TODO
const STRING_PARAM_KEY: u8 = 39;
/*const DAYS_SINCE_ACTIVE_PARAM_KEY: u8 = 40;
const START_RANGE_PARAM_KEY: u8 = 41;
const END_RANGE_PARAM_KEY: u8 = 43;
const TYPES_PARAM_KEY: u8 = 34;*/

// params out
const RESULTS_PARAM_KEY: u8 = 42;

pub(super) fn search_clans_provider() -> SimpleFunc<32, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(s)) = params.get(&STRING_PARAM_KEY) {
            if !s.string.is_empty() {
                log::debug!("Got clan search string `{}`", s.string);
            }
        }
        params.insert(RESULTS_PARAM_KEY, Typed::Arr(Arr {
            ty: 104, // hashmap
            items: vec![
                ClanInfo {
                    clan_name: "".to_owned(),
                    clan_description: "".to_owned(),
                    clan_type: ClanType::Closed,
                    clan_size: 1,
                }.as_transmissible(),
            ],
        }));
        Ok(params.into())
    })
}
