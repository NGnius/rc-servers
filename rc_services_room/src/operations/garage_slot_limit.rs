use polariton_server::operations::Immediate;

//use oj_rc_core::ConfigProvider;

const PARAM_KEY: u8 = 68;

pub(super) fn garage_slots_limit(conf: &oj_rc_core::ConfigImpl) -> Immediate<60, crate::UserTy> {
    let limit = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::garage_slot_limit(conf);
    Immediate::new(move || {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, polariton::operation::Typed::Int(limit));
        params.into()
    })
}
