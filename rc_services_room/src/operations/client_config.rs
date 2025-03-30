use polariton_server::operations::Immediate;
//use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};
use rc_core::ConfigProvider;

const PARAM_KEY: u8 = 36;

pub(super) fn client_config_provider(conf: &rc_core::ConfigImpl) -> Immediate<34, crate::UserTy> {
    let client_conf = conf.client_config();
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, client_conf.clone());
        params.into()
    })
}
