use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 44;

const SLOT_PARAM_KEY: u8 = 48; // uint; in

pub(super) fn garage_slot_selector() -> SlotSelector {
    SlotSelector
}

async fn do_select(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    let user_info = user.user()?;
    if let Some(Typed::Int(to_select)) = params.remove(&SLOT_PARAM_KEY) {
        user_info.select_garage(to_select).await?;
    }
    Ok(params.into())
}

pub struct SlotSelector;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for SlotSelector {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_select(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for SlotSelector {
    fn op_code() -> u8 {
        CODE
    }
}
