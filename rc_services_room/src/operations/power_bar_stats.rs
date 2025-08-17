use polariton_server::operations::Immediate;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 61;

pub(super) fn power_bar_provider(conf: &oj_rc_core::ConfigImpl) -> Immediate<51, crate::UserTy> {
    let energy_conf = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::energy(conf);
    Immediate::new(|| {
        let mut params = ParameterTable::with_capacity(2);
        params.insert(PARAM_KEY, Typed::HashMap(vec![
                (Typed::Str("refillRatePerSecond".into()), Typed::Float(energy_conf.refill_rate)),
                (Typed::Str("powerForAllRobots".into()), Typed::Int(energy_conf.total as _)),
            ].into()
        ));
        params
    })
}
