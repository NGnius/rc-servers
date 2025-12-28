use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 55;

// const USERNAME_PARAM_KEY: u8 = 1; // in; string
const PARAM_KEY: u8 = 60;

pub(super) struct ClaimPreviousBattleRewards;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for ClaimPreviousBattleRewards {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, _params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut table = ParameterTable::with_capacity(2);
        let user_info = user.user()?;
        let is_success = user_info.claim_match_rewards().await?;
        table.insert(PARAM_KEY, Typed::Bool(!is_success || user_info.has_unclaimed_match_rewards().await?));
        Ok(table)
    }
}

pub(super) fn claim_battle_rewards_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, ClaimPreviousBattleRewards> {
    SimpleOpImpl::new(ClaimPreviousBattleRewards)
}

