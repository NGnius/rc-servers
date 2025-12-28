use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 83;

const PARAM_KEY: u8 = 8;

pub(super) struct UserTotalExperiencer;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for UserTotalExperiencer {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, _params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut table = ParameterTable::with_capacity(2);
        let user_info = user.user()?;
        let exp = user_info.currency(
            oj_rc_core::persist::user::CurrencyType::Experience,
            oj_rc_core::persist::user::CurrencyOp::Get,
        ).await?;
        table.insert(PARAM_KEY, Typed::Int(exp as _));
        Ok(table)
    }
}

pub(super) fn get_user_xp_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, UserTotalExperiencer> {
    SimpleOpImpl::new(UserTotalExperiencer)
}
