use polariton_server::operations::Immediate;
use oj_rc_core::persist::config::ConfigProviderC;

const PARAM_KEY: u8 = 57;

pub(super) fn weapon_config_provider(cubes: &oj_rc_core::ConfigImpl) -> Immediate<47, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, cubes.weapon_list());
        params.into()
    })
}
