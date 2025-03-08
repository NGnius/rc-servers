use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

//const PLATOON_ID_PARAM_KEY: u8 = 16;
//const PLATOON_LEADER_PARAM_KEY: u8 = 17;
//const USER_LIST_PARAM_KEY: u8 = 7;

pub(super) fn platoon_provider<C: Send + Sync>() -> SimpleFunc<18, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        //let mut params = params.to_dict();
        // if platoon ID is not provided, you're not in a platoon
        //Ok(params.into())
        Ok(params)
    })
}
