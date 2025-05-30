use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 16;

const SANCTIONS_PARAM_KEY: u8 = 31; // arr of str; out
const USERNAME_PARAM_KEY: u8 = 22; // str; in

pub(super) struct GetSanctionsProvider;

#[async_trait::async_trait]
impl SimpleOperation<()> for GetSanctionsProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable, user: &Self::User) -> Result<ParameterTable, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            let user_info = user.user()?;
            let sanctions = user_info.get_sanctions(username.string).await?;
            params.insert(SANCTIONS_PARAM_KEY, sanctions);
            return Ok(params.into());
        }
        Err((rc_core::data::error_codes::ChatErrorCodes::UnexpectedError as i16).into())
    }
}

pub(super) fn list_sanctions_provider() -> SimpleOpImpl<(), crate::UserTy, GetSanctionsProvider> {
    SimpleOpImpl::new(GetSanctionsProvider)
}
