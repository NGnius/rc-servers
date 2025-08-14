use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const USERNAME_PARAM_KEY: u8 = 22; // str; in & out
const DISPLAY_NAME_PARAM_KEY: u8 = 30; // str; out
const CAN_SEND_DM_PARAM_KEY: u8 = 27; // int; out

#[allow(dead_code)]
#[repr(u8)]
enum CanSendMessageResult {
    Ok = 0,
    UserDoesNotExist = 1,
    UserOffline = 2,
}

const CODE: u8 = 14;

pub(super) struct OnlineChecker {
    chat_system: crate::state::chat::ChatImpl
}

pub(super) fn is_online_provider(chat_system: crate::state::chat::ChatImpl) -> SimpleOpImpl<(), crate::UserTy, OnlineChecker> {
    SimpleOpImpl::new(OnlineChecker {
        chat_system,
    })
}

#[async_trait::async_trait]
impl SimpleOperation<()> for OnlineChecker {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<()>, _user: &Self::User) -> Result<ParameterTable<()>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(user_name)) = params.get(&USERNAME_PARAM_KEY) {
            let username = user_name.string.clone();
            params.insert(DISPLAY_NAME_PARAM_KEY, Typed::Str(username.clone().into()));
            if self.chat_system.system().await.is_user_online(&username) {
                params.insert(CAN_SEND_DM_PARAM_KEY, Typed::Int(CanSendMessageResult::Ok as _));
                Ok(params.into())
            } else {
                log::debug!("User {} is not online", username);
                params.insert(CAN_SEND_DM_PARAM_KEY, Typed::Int(CanSendMessageResult::UserOffline as _));
                //params.insert(CAN_SEND_DM_PARAM_KEY, Typed::Int(CanSendMessageResult::Ok as _));
                Ok(params.into())
            }
        } else {
            Err((oj_rc_core::data::error_codes::ChatErrorCodes::UnexpectedError as i16).into())
        }
    }
}
