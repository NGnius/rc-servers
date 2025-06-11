use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 3;

const DID_QUIT_LAST_GAME_PARAM_KEY: u8 = 19; // bool
const BLOCK_TIME_PARAM_KEY: u8 = 15; // int

pub(super) struct QuitterBlockProvider;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for QuitterBlockProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, _user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        params.insert(DID_QUIT_LAST_GAME_PARAM_KEY, Typed::Bool(false));
        params.insert(BLOCK_TIME_PARAM_KEY, Typed::Int(0));
        Ok(params.into())
    }
}

pub(super) fn quit_blocker_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, QuitterBlockProvider> {
    SimpleOpImpl::new(QuitterBlockProvider)
}
