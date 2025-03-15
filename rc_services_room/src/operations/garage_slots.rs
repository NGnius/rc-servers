use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const SLOTS_PARAM_KEY: u8 = 44;
const SELECTED_SLOT_PARAM_KEY: u8 = 43;
const SLOT_ORDER_PARAM_KEY: u8 = 58;

pub(super) fn garage_slot_provider() -> SimpleFunc<40, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let lock = user.read().unwrap();
        let user_info = lock.user()?;
        let mut params = params.to_dict();
        let all_slots = user_info.all_slots_by_id();
        params.insert(SLOTS_PARAM_KEY, all_slots.slot_info);
        params.insert(SELECTED_SLOT_PARAM_KEY, Typed::Int(user_info.selected_garage_slot() as _));
        params.insert(SLOT_ORDER_PARAM_KEY, all_slots.slot_order);
        Ok(params.into())
    })
}
