use polariton_server::operations::Immediate;
use rc_core::ConfigProvider;

const PARAM_KEY: u8 = 38;

pub(super) fn weapons_upgrade_provider(cubes: &rc_core::ConfigImpl) -> Immediate<82, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, cubes.weapon_upgrade_list());
        params.into()
    })
}
