use polariton::operation::{ParameterTable, Typed};
use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};

const JOIN_CODE: u8 = 1;
const LEAVE_CODE: u8 = 5;

const CHANNEL_NAME_PARAM_KEY: u8 = 3; // str; in
const CHANNEL_TYPE_PARAM_KEY: u8 = 1; // int; in
const CHANNEL_PASSWORD_PARAM_KEY: u8 = 16; // str; in
const CHANNEL_INFO_PARAM_KEY: u8 = 17; // hashtable; out

pub struct JoinChannelProvider {
    chat_system: crate::state::ChatImpl,
}

pub(super) fn join_channel_provider(chat_system: crate::state::ChatImpl) -> SimpleOpImpl<(), crate::UserTy, JoinChannelProvider> {
    SimpleOpImpl::new(JoinChannelProvider { chat_system })
}

#[async_trait::async_trait]
impl SimpleOperation<()> for JoinChannelProvider {
    type User = crate::UserTy;
    const CODE: u8 = JOIN_CODE;

    async fn handle(&self, params: ParameterTable<()>, user: &Self::User) -> Result<ParameterTable<()>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(chann_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
            if let Some(Typed::Int(chann_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
                if let Some(Typed::Str(channel_pwd)) = params.remove(&CHANNEL_PASSWORD_PARAM_KEY) {
                    log::warn!("Received channel password {} which is unsupported", channel_pwd.string);
                }
                let user_info = user.user()?;
                //let chat_user = super::get_chat_user(user_info.as_ref().as_ref());
                self.chat_system.system_mut().await.join_channel(user_info.public_id().to_owned(), chann_name.string.clone());
                let response = user_info.add_subscribed_channel(chann_name.string, crate::data::channel::ChatChannelType::from_u8(chann_ty as _)?).await?;
                params.insert(CHANNEL_INFO_PARAM_KEY, response);
            }
        }
        Ok(params.into())
    }
}

pub struct LeaveChannelProvider {
    chat_system: crate::state::ChatImpl,
}

pub(super) fn leave_channel_provider(chat_system: crate::state::ChatImpl) -> SimpleOpImpl<(), crate::UserTy, LeaveChannelProvider> {
    SimpleOpImpl::new(LeaveChannelProvider { chat_system })
}

#[async_trait::async_trait]
impl SimpleOperation<()> for LeaveChannelProvider {
    type User = crate::UserTy;
    const CODE: u8 = LEAVE_CODE;

    async fn handle(&self, params: ParameterTable<()>, user: &Self::User) -> Result<ParameterTable<()>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(chann_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
            if let Some(Typed::Int(chann_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
                let user_info = user.user()?;
                //let chat_user = super::get_chat_user(user_info.as_ref().as_ref());
                self.chat_system.system_mut().await.leave_channel(user_info.public_id().to_owned(), chann_name.string.clone());
                user_info.remove_subscribed_channel(chann_name.string, crate::data::channel::ChatChannelType::from_u8(chann_ty as _)?).await?;
            }
        }
        Ok(params.into())
    }
}
