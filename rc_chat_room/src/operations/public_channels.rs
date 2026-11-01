use polariton_server::operations::Immediate;
use oj_rc_core::persist::config::ConfigProviderC;

const PARAM_KEY: u8 = 20;

pub(super) fn public_channels_provider(conf: &oj_rc_core::persist::config::ConfigImpl) -> Immediate<13, crate::UserTy> {
    let pub_channs = conf.public_channels();
    Immediate::new(move || {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, pub_channs.to_owned());
        params.into()
    })
}
