use crate::{persist::chat_user::ChatUser, SimpleChatFunc};
use polariton::operation::{ParameterTable, Typed};

const CHANNEL_NAME_PARAM_KEY: u8 = 3; // str; in
const CHANNEL_TYPE_PARAM_KEY: u8 = 1; // int; in
const CHANNEL_PASSWORD_PARAM_KEY: u8 = 16; // str; in
const CHANNEL_INFO_PARAM_KEY: u8 = 17; // hashtable; out

pub(super) fn join_channel_provider(chat_system: crate::state::ChatImpl) -> SimpleChatFunc<1, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy, &crate::state::ChatImpl) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleChatFunc::new(|params, user: &crate::UserTy, chat_system| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(chann_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
            if let Some(Typed::Int(chann_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
                if let Some(Typed::Str(channel_pwd)) = params.remove(&CHANNEL_PASSWORD_PARAM_KEY) {
                    log::warn!("Received channel password {} which is unsupported", channel_pwd.string);
                }
                let user_info = user.user()?;
                let chat_user = super::get_chat_user(user_info.as_ref().as_ref());
                chat_system.system_mut().join_channel(user_info.token().uuid.clone(), chann_name.string.clone());
                let response = chat_user.add_subscribed_channel(chann_name.string, crate::data::channel::ChatChannelType::from_u8(chann_ty as _)?);
                params.insert(CHANNEL_INFO_PARAM_KEY, response);
            }
        }
        Ok(params.into())
    }, chat_system)
}

pub(super) fn leave_channel_provider(chat_system: crate::state::ChatImpl) -> SimpleChatFunc<5, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy, &crate::state::ChatImpl) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleChatFunc::new(|params, user: &crate::UserTy, chat_system| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(chann_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
            if let Some(Typed::Int(chann_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
                let user_info = user.user()?;
                let chat_user = super::get_chat_user(user_info.as_ref().as_ref());
                chat_system.system_mut().leave_channel(user_info.token().uuid.clone(), chann_name.string.clone());
                chat_user.remove_subscribed_channel(chann_name.string, crate::data::channel::ChatChannelType::from_u8(chann_ty as _)?);
            }
        }
        Ok(params.into())
    }, chat_system)
}
