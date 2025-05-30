use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 15;

const PARAM_KEY: u8 = 28;

pub(super) struct PendingSanctionsProvider;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for PendingSanctionsProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, _user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        //let user_info = user.user()?;
        //params.insert(PARAM_KEY, Typed::Bool(user_info.has_pending_sanctions().await?));
        params.insert(PARAM_KEY, Typed::Bool(false));
        Ok(params.into())
    }
}

pub(super) fn pending_sanctions_checker<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, PendingSanctionsProvider> {
    SimpleOpImpl::new(PendingSanctionsProvider)
}
