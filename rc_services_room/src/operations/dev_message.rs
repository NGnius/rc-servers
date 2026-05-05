use polariton_server::operations::{Operation, OperationCode};
use rand::RngExt;
use oj_rc_core::ConfigProvider;

const MAX_DELTA: u64 = 60 * 60 * 24; // 1 day

const CODE: u8 = 8;

const MESSAGE_PARAM_KEY: u8 = 2;
const DISPLAY_TIME_PARAM_KEY: u8 = 15;

pub struct DevMessageProvider<C: Clone> {
    messages: oj_rc_core::persist::config::DevMessageProvider<C>,
}

impl <C: Clone> DevMessageProvider<C> {
    async fn do_handling(&self, params: polariton::operation::ParameterTable<C>, user: &crate::UserTy) -> Result<polariton::operation::ParameterTable<C>, i16> {
        let mut params = params.to_dict();
        let user_info = user.user()?;
        let last_seen = user_info.last_seen().await?;
        let now = chrono::Utc::now().timestamp() as u64;
        if (now - last_seen) > MAX_DELTA {
            let index = rand::rng().random::<u32>();
            let dev_msg = self.messages.get(index as _);
            params.insert(MESSAGE_PARAM_KEY, dev_msg.message);
            params.insert(DISPLAY_TIME_PARAM_KEY, dev_msg.display_time);
        } else {
            let dev_msg = self.messages.get_empty();
            params.insert(MESSAGE_PARAM_KEY, dev_msg.message);
            params.insert(DISPLAY_TIME_PARAM_KEY, dev_msg.display_time);
        }

        Ok(params.into())
    }
}

#[async_trait::async_trait]
impl <C: Clone + Send + Sync + 'static> Operation<C> for DevMessageProvider<C> {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<C>, user: &Self::User) -> polariton::operation::OperationResponse<C> {
        polariton_server::operations::result_to_op_resp::<CODE, C>(self.do_handling(params, user).await)
    }
}

impl <C: Clone> OperationCode for DevMessageProvider<C> {
    fn op_code() -> u8 {
        CODE
    }
}

pub(super) fn dev_message_provider(conf: &oj_rc_core::ConfigImpl) -> DevMessageProvider<()> {
    let messages = conf.login_messages();
    DevMessageProvider { messages }
}
