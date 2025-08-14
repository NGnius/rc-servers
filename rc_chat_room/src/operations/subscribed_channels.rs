//use oj_rc_core::persist::user::ChatUser;
use polariton::operation::{ParameterTable, OperationResponse};

const CODE: u8 = 12; // get all subscribed

const PARAM_KEY: u8 = 18;

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    log::info!("Getting subscribed user's channels");
    let mut params = params.to_dict();
    let user_info = user.user()?;
    params.insert(PARAM_KEY, user_info.subscribed_channels().await?);
    Ok(params.into())
}

pub struct SubscribedChannelsProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for SubscribedChannelsProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for SubscribedChannelsProvider {
    fn op_code() -> u8 {
        CODE
    }
}


pub(super) fn all_subbed_channels_provider() -> SubscribedChannelsProvider {
    SubscribedChannelsProvider
}
