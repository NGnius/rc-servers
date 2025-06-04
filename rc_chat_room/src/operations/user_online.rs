use crate::SimpleChatFunc;
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


pub(super) fn is_online_provider(chat_system: crate::state::ChatImpl) -> SimpleChatFunc<14, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy, &crate::state::ChatImpl) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleChatFunc::new(|params, _: &crate::UserTy, chat_system| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(user_name)) = params.get(&USERNAME_PARAM_KEY) {
            let username = user_name.string.clone();
            params.insert(DISPLAY_NAME_PARAM_KEY, Typed::Str(username.clone().into()));
            if chat_system.system().is_user_online(&username) {
                params.insert(CAN_SEND_DM_PARAM_KEY, Typed::Int(CanSendMessageResult::Ok as _));
                Ok(params.into())
            } else {
                log::debug!("User {} is not online", username);
                params.insert(CAN_SEND_DM_PARAM_KEY, Typed::Int(CanSendMessageResult::UserOffline as _));
                //params.insert(CAN_SEND_DM_PARAM_KEY, Typed::Int(CanSendMessageResult::Ok as _));
                Ok(params.into())
            }
        } else {
            Err(oj_rc_core::data::error_codes::ChatErrorCodes::UnexpectedError as _)
        }
    }, chat_system)
}
