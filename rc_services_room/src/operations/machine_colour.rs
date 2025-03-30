use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

// const USERNAME_PARAM_KEY: u8 = 30; // str
const SLOT_PARAM_KEY: u8 = 31; // int
const DATA_PARAM_KEY: u8 = 33; // byte arr

pub(super) fn garage_machine_colour_provider() -> SimpleFunc<33, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let mut params = params.to_dict();
        let user_info = user.user()?;
        if let Some(Typed::Int(garage_slot)) = params.get(&SLOT_PARAM_KEY) {
            log::debug!("Got machine colour request for slot {:?}", garage_slot);
            let machine = user_info.slot_by_id(*garage_slot)?;
            params.insert(DATA_PARAM_KEY, machine.colour_data);
        } else {
            params.insert(SLOT_PARAM_KEY, Typed::Int(user_info.selected_garage_slot() as _));
        }

        Ok(params.into())
    })
}
