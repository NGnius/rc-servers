use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const FREE_BALANCE_PARAM_KEY: u8 = 74;
const PAID_BALANCE_PARAM_KEY: u8 = 87;

const CODE: u8 = 66;

pub(super) struct WalletBallancer;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for WalletBallancer {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, _params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = ParameterTable::with_capacity(3);
        let user_info = user.user()?;
        let free = user_info.currency(
            oj_rc_core::persist::user::CurrencyType::Free,
            oj_rc_core::persist::user::CurrencyOp::Get,
        ).await?;
        let paid = user_info.currency(
            oj_rc_core::persist::user::CurrencyType::Paid,
            oj_rc_core::persist::user::CurrencyOp::Get,
        ).await?;
        params.insert(FREE_BALANCE_PARAM_KEY, Typed::Long(free as _));
        params.insert(PAID_BALANCE_PARAM_KEY, Typed::Long(paid as _));
        Ok(params)
    }
}

pub(super) fn balance_wallet_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, WalletBallancer> {
    SimpleOpImpl::new(WalletBallancer)
}

