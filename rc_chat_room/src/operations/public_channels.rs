use polariton_server::operations::Immediate;
//use polariton::operation::{ParameterTable, Typed};
use oj_rc_core::ConfigProvider;

const PARAM_KEY: u8 = 20;

pub(super) fn public_channels_provider(conf: &oj_rc_core::persist::config::ConfigImpl) -> Immediate<13, crate::UserTy> {
    let pub_channs = conf.public_channels();
    Immediate::new(move || {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, pub_channs.to_owned());
        /*params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::Str,
            items: vec![
                Typed::Str("Pluto".into()),
            ],
        }));*/
        params.into()
    })
}
