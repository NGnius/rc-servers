use polariton_server::operations::Immediate;

use crate::persist::config::ConfigProvider;

pub const DEFAULT_WEAPON_ORDER_PARAM_KEY: u8 = 138;

pub(super) fn weapon_order_provider(conf: &crate::persist::config::ConfigImpl) -> Immediate<118, crate::UserTy> {
    let weapon_orders = conf.weapon_keys();
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(DEFAULT_WEAPON_ORDER_PARAM_KEY, weapon_orders.clone());
        params.into()
    })
}
