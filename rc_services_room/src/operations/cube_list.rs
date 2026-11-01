use polariton_server::operations::Immediate;
use oj_rc_core::persist::config::ConfigProviderC;

const PARAM_KEY: u8 = 1;
//const DEFAULT_CUBE_ID: u32 = 227205318;

pub(super) fn cube_list_provider(cubes: &oj_rc_core::ConfigImpl) -> Immediate<2, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, cubes.cube_list());
        params.into()
    })
}
