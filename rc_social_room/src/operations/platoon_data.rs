use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

//const PLATOON_ID_PARAM_KEY: u8 = 16;
//const PLATOON_LEADER_PARAM_KEY: u8 = 17;
//const USER_LIST_PARAM_KEY: u8 = 7;

pub(super) fn platoon_provider() -> SimpleFunc<18, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        //let mut params = params.to_dict();
        // if platoon ID is not provided, you're not in a platoon
        //Ok(params.into())
        Ok(params)
    })
}
