//use oj_rc_core::persist::user::ChatUser;
use polariton::operation::{ParameterTable, OperationResponse};

const CODE: u8 = 11; // subscribe all

const PARAM_KEY: u8 = 18;

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy, chat_system: &crate::state::chat::ChatImpl) -> Result<ParameterTable, i16> {
    log::info!("Adding joined user to channels");
    let mut params = params.to_dict();
    let user_info = user.user()?;
    let name = user_info.public_id().to_owned();
    let channels = user_info.subscribed_channels_strings().await?;
    let event_tx = user.event_chann();
    chat_system.system_mut().await.connect_user(name, channels, event_tx);
    params.insert(PARAM_KEY, user_info.subscribed_channels().await?);
    Ok(params.into())
}

pub struct JoinedChannelsProvider {
    chat_system: crate::state::chat::ChatImpl,
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for JoinedChannelsProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, &self.chat_system).await)
    }
}

impl polariton_server::operations::OperationCode for JoinedChannelsProvider {
    fn op_code() -> u8 {
        CODE
    }
}


pub(super) fn all_channels_provider(chat_system: crate::state::chat::ChatImpl) -> JoinedChannelsProvider {
    JoinedChannelsProvider {
        chat_system,
    }
}
