use polariton::operation::{ParameterTable, Typed, OperationResponse};

const JOIN_CODE: u8 = 1;
const LEAVE_CODE: u8 = 5;

const CHANNEL_NAME_PARAM_KEY: u8 = 3; // str; in
const CHANNEL_TYPE_PARAM_KEY: u8 = 1; // int; in
const CHANNEL_PASSWORD_PARAM_KEY: u8 = 16; // str; in
const CHANNEL_INFO_PARAM_KEY: u8 = 17; // hashtable; out

pub struct JoinChannelProvider {
    chat_system: crate::state::ChatImpl,
}

pub(super) fn join_channel_provider(chat_system: crate::state::ChatImpl) -> JoinChannelProvider {
    JoinChannelProvider { chat_system }
}

async fn do_join_handling(params: ParameterTable<()>, user: &crate::UserTy, chat_system: &crate::state::ChatImpl) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Str(chann_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
        if let Some(Typed::Int(chann_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
            if let Some(Typed::Str(channel_pwd)) = params.remove(&CHANNEL_PASSWORD_PARAM_KEY) {
                log::warn!("Received channel password {} which is unsupported", channel_pwd.string);
            }
            let user_info = user.user()?;
            //let chat_user = super::get_chat_user(user_info.as_ref().as_ref());
            chat_system.system_mut().join_channel(user_info.public_id().to_owned(), chann_name.string.clone());
            let response = user_info.add_subscribed_channel(chann_name.string, crate::data::channel::ChatChannelType::from_u8(chann_ty as _)?).await?;
            params.insert(CHANNEL_INFO_PARAM_KEY, response);
        }
    }
    Ok(params.into())
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for JoinChannelProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<JOIN_CODE, ()>(do_join_handling(params, user, &self.chat_system).await)
    }
}

impl polariton_server::operations::OperationCode for JoinChannelProvider {
    fn op_code() -> u8 {
        JOIN_CODE
    }
}

pub struct LeaveChannelProvider {
    chat_system: crate::state::ChatImpl,
}

pub(super) fn leave_channel_provider(chat_system: crate::state::ChatImpl) -> LeaveChannelProvider {
    LeaveChannelProvider { chat_system }
}

async fn do_leave_handling(params: ParameterTable<()>, user: &crate::UserTy, chat_system: &crate::state::ChatImpl) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Str(chann_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
        if let Some(Typed::Int(chann_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
            let user_info = user.user()?;
            //let chat_user = super::get_chat_user(user_info.as_ref().as_ref());
            chat_system.system_mut().leave_channel(user_info.public_id().to_owned(), chann_name.string.clone());
            user_info.remove_subscribed_channel(chann_name.string, crate::data::channel::ChatChannelType::from_u8(chann_ty as _)?).await?;
        }
    }
    Ok(params.into())
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for LeaveChannelProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<LEAVE_CODE, ()>(do_leave_handling(params, user, &self.chat_system).await)
    }
}

impl polariton_server::operations::OperationCode for LeaveChannelProvider {
    fn op_code() -> u8 {
        LEAVE_CODE
    }
}
