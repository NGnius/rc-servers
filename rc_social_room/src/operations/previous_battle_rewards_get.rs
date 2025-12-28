use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 53;

// const USERNAME_PARAM_KEY: u8 = 1; // in; string
//const PARAM_KEY: u8 = 60;
const NEW_SEASON_XP_PARAM_KEY: u8 = 57; // int; out
const XP_AWARD_BASE_PARAM_KEY: u8 = 58; // int; out
const XP_AWARD_PREMIUM_PARAM_KEY: u8 = 59; // int; out
const XP_AWARD_PARTY_PARAM_KEY: u8 = 61; // int; out
const XP_AWARD_TIER_PARAM_KEY: u8 = 52; // int; out
const ROBITS_TOTAL_PARAM_KEY: u8 = 50; // int; out
const AVERAGE_XP_PARAM_KEY: u8 = 54; // int; out
const CLAN_TOTAL_XP_PARAM_KEY: u8 = 55; // int; out
const ROBITS_PARAM_KEY: u8 = 68; // int; out
const PREMIUM_ROBITS_PARAM_KEY: u8 = 69; // int; out
const LONG_PLAY_MULTIPLIER_PARAM_KEY: u8 = 72; // float; out

pub(super) struct GetPreviousBattleRewards;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for GetPreviousBattleRewards {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, _params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut table = ParameterTable::with_capacity(12);
        let user_info = user.user()?;
        let next_rewards = user_info.get_unclaimed_match_rewards().await?;
        table.insert(NEW_SEASON_XP_PARAM_KEY, Typed::Int(next_rewards.season_experience));
        table.insert(XP_AWARD_BASE_PARAM_KEY, Typed::Int(next_rewards.experience_award_base));
        table.insert(XP_AWARD_PREMIUM_PARAM_KEY, Typed::Int(next_rewards.experience_award_premium));
        table.insert(XP_AWARD_PARTY_PARAM_KEY, Typed::Int(next_rewards.experience_award_party));
        table.insert(XP_AWARD_TIER_PARAM_KEY, Typed::Int(next_rewards.experience_award_tier));
        table.insert(ROBITS_TOTAL_PARAM_KEY, Typed::Int(next_rewards.robits_total));
        table.insert(AVERAGE_XP_PARAM_KEY, Typed::Int(next_rewards.average_experience));
        table.insert(CLAN_TOTAL_XP_PARAM_KEY, Typed::Int(next_rewards.clan_experience));
        table.insert(ROBITS_PARAM_KEY, Typed::Int(next_rewards.robits_earned));
        table.insert(PREMIUM_ROBITS_PARAM_KEY, Typed::Int(next_rewards.premium_robits_earned));
        table.insert(LONG_PLAY_MULTIPLIER_PARAM_KEY, Typed::Float(1.0));
        Ok(table)
    }
}

pub(super) fn get_battle_rewards_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, GetPreviousBattleRewards> {
    SimpleOpImpl::new(GetPreviousBattleRewards)
}

