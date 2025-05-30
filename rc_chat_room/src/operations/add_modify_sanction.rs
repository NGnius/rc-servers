use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 7;

const SANCTION_TY_PARAM_KEY: u8 = 9; // int; in
const IS_ADDING_PARAM_KEY: u8 = 10; // bool; in
const DURATION_PARAM_KEY: u8 = 11; // int; in
const REASON_PARAM_KEY: u8 = 2; // str; in
const USERNAME_PARAM_KEY: u8 = 7; // str; in

pub(super) struct AddSanctionProvider;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for AddSanctionProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Int(sanction_ty)) = params.remove(&SANCTION_TY_PARAM_KEY) {
            if let Some(Typed::Bool(is_adding)) = params.remove(&IS_ADDING_PARAM_KEY) {
                if let Some(Typed::Int(duration)) = params.remove(&DURATION_PARAM_KEY) {
                    if let Some(Typed::Str(reason)) = params.remove(&REASON_PARAM_KEY) {
                        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
                            let user_info = user.user()?;
                            let sanction = rc_core::persist::user::SetSanction {
                                type_: rc_core::persist::user::SanctionType::from_i32(sanction_ty)?,
                                is_adding,
                                duration,
                                reason: reason.string,
                                username: username.string,
                            };
                            user_info.set_sanction(sanction).await?;
                            return Ok(params.into());
                        }
                    }
                }
            }
        }
        Err((rc_core::data::error_codes::ChatErrorCodes::UnexpectedError as i16).into())
    }
}

pub(super) fn add_modify_sanction_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, AddSanctionProvider> {
    SimpleOpImpl::new(AddSanctionProvider)
}
