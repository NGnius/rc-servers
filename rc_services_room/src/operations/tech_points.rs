use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};
use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};

const CURRENT_CODE: u8 = 187;
const CURRENT_PARAM_KEY: u8 = 214;

pub(super) struct CurrentTechPointers;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CurrentTechPointers {
    type User = crate::UserTy;
    const CODE: u8 = CURRENT_CODE;

    async fn handle(&self, _params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut table = ParameterTable::with_capacity(2);
        let user_info = user.user()?;
        let exp = user_info.currency(
            oj_rc_core::persist::user::CurrencyType::TechPoints,
            oj_rc_core::persist::user::CurrencyOp::Get,
        ).await?;
        table.insert(CURRENT_PARAM_KEY, Typed::Int(exp as _));
        Ok(table)
    }
}

pub(super) fn tech_points_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, CurrentTechPointers> {
    SimpleOpImpl::new(CurrentTechPointers)
}


const UNCLAIMED_PARAM_KEY: u8 = 212;

pub(super) fn tech_points_awards_provider() -> SimpleFunc<185, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(UNCLAIMED_PARAM_KEY, Typed::Int(0));
        Ok(params.into())
    })
}
