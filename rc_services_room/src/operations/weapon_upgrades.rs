use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::weapon_upgrade::*;

const PARAM_KEY: u8 = 38;

pub(super) fn weapons_upgrade_provider() -> SimpleFunc<82, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::ObjArr(vec![
            WeaponUpgradeInfo {
                tier: crate::data::cube_list::ItemTier::T0,
                type_: crate::data::weapon_list::ItemCategory::Laser,
                xp: 4.2,
                rating: 1,
                rank: 1,
                power: 1,
            }.as_transmissible(),
        ].into()));
        Ok(params.into())
    })
}
