use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const MONTH_PARAM_KEY: u8 = 49;
const YEAR_PARAM_KEY: u8 = 63;
const ROBITS_PARAM_KEY: u8 = 47;
const IS_CLAIMED_PARAM_KEY: u8 = 46;
const CLAN_AVERAGE_PARAM_KEY: u8 = 54;
const CLAN_TOTAL_PARAM_KEY: u8 = 55;
const CLAN_NAME_PARAM_KEY: u8 = 31;
const PLAYER_XP_PARAM_KEY: u8 = 57;

pub(super) fn season_rewards_provider<C: Send + Sync>() -> SimpleFunc<50, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(MONTH_PARAM_KEY, Typed::Int(2));
        params.insert(YEAR_PARAM_KEY, Typed::Int(2025));
        params.insert(ROBITS_PARAM_KEY, Typed::Int(42));
        params.insert(IS_CLAIMED_PARAM_KEY, Typed::Bool(true));
        params.insert(CLAN_AVERAGE_PARAM_KEY, Typed::Int(67));
        params.insert(CLAN_TOTAL_PARAM_KEY, Typed::Int(42_123));
        params.insert(CLAN_NAME_PARAM_KEY, Typed::Str("RE_clan_name_rewards".into()));
        params.insert(PLAYER_XP_PARAM_KEY, Typed::Int(10_000));
        Ok(params.into())
    })
}
