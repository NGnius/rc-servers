use polariton_server::operations::Immediate;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 100;

pub(super) fn robot_shop_submission_infos_provider(config: &oj_rc_core::ConfigImpl) -> Immediate<95, crate::UserTy> {
    let max_subs = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::factory_config(config).upload_limit;
    Immediate::new(|| {
        let mut params = ParameterTable::with_capacity(2);
        params.insert(PARAM_KEY, Typed::HashMap(vec![
                (Typed::Str("submissionCount".into()), Typed::Int(0)),
                (Typed::Str("maxSubmissions".into()), Typed::Int(max_subs)),
            ].into()));
        params
    })
}
