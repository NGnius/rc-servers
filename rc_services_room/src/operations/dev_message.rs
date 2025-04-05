use polariton_server::operations::{Operation, OperationCode};
use rand::Rng;
use rc_core::ConfigProvider;

const MESSAGE_PARAM_KEY: u8 = 2;
const DISPLAY_TIME_PARAM_KEY: u8 = 15;

pub struct DevMessageProvider<C: Clone> {
    messages: rc_core::persist::config::DevMessageProvider<C>,
}

impl <C: Clone + Send + Sync> Operation<C> for DevMessageProvider<C> {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, params: polariton::operation::ParameterTable<C>, _: &mut Self::State, _user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut params = params.to_dict();
        let index = rand::rng().random::<u32>();
        let dev_msg = self.messages.get(index as _);
        params.insert(MESSAGE_PARAM_KEY, dev_msg.message);
        params.insert(DISPLAY_TIME_PARAM_KEY, dev_msg.display_time);
        polariton::operation::OperationResponse {
            code: Self::op_code(),
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params: params.into(),
        }
    }
}

impl <C: Clone> OperationCode for DevMessageProvider<C> {
    fn op_code() -> u8 {
        16
    }
}

pub(super) fn dev_message_provider(conf: &rc_core::ConfigImpl) -> DevMessageProvider<()> {
    let messages = conf.login_messages();
    DevMessageProvider { messages }
}
