use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;
use rand::Rng;
use rc_core::ConfigProvider;

const MESSAGE_PARAM_KEY: u8 = 2;
const DISPLAY_TIME_PARAM_KEY: u8 = 15;

pub(super) fn dev_message_provider(conf: &rc_core::ConfigImpl) -> SimpleFunc<8, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    let messages = conf.login_messages();
    SimpleFunc::new(move |params, _| {
        let mut params = params.to_dict();
        let index = rand::rng().random::<u32>();
        let dev_msg = messages.get(index as _);
        params.insert(MESSAGE_PARAM_KEY, dev_msg.message);
        params.insert(DISPLAY_TIME_PARAM_KEY, dev_msg.display_time);
        Ok(params.into())
    })
}
