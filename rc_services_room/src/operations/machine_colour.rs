use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

// const USERNAME_PARAM_KEY: u8 = 30; // str
const SLOT_PARAM_KEY: u8 = 31; // int
const DATA_PARAM_KEY: u8 = 33; // byte arr

pub(super) fn garage_machine_colour_provider() -> SimpleFunc<33, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(garage_slot) = params.get(&SLOT_PARAM_KEY) {
            log::debug!("Got machine colour request for slot {:?}", garage_slot);
        } else {
            params.insert(SLOT_PARAM_KEY, Typed::Int(0));
        }
        params.insert(DATA_PARAM_KEY, Typed::Bytes(vec![0u8, 0u8, 0u8, 0u8].into())); // first 4 bytes are i32 for length of rest of data
        Ok(params.into())
    })
}
