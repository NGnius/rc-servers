use polariton_server::operations::Immediate;

//use oj_rc_core::ConfigProvider;

const PARAM_KEY: u8 = 68;

pub(super) fn garage_slots_limit(_conf: &oj_rc_core::ConfigImpl) -> Immediate<60, crate::UserTy> {
    //let limit = conf.game_mode_config();
    Immediate::new(move || {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(PARAM_KEY, polariton::operation::Typed::Int(100));
        params.into()
    })
}
