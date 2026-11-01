use oj_rc_core::ConfigProvider;
use polariton_server::operations::Immediate;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 207;

pub(super) fn available_reconnect_provider(conf: &oj_rc_core::ConfigImpl) -> Immediate<171, crate::UserTy> {
    let reconnect_enabled = conf.reconnect().is_some();
    Immediate::new(move || {
        let mut params = ParameterTable::with_capacity(2);
        params.insert(PARAM_KEY, Typed::Bool(reconnect_enabled));
        params
    })
}
