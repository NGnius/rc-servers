use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 40;

const SLOTS_PARAM_KEY: u8 = 44;
const SELECTED_SLOT_PARAM_KEY: u8 = 43;
const SLOT_ORDER_PARAM_KEY: u8 = 58;

pub(super) fn garage_slot_provider() -> GarageSlotsProvider {
    GarageSlotsProvider
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let user_info = user.user()?;
    let mut params = params.to_dict();
    let all_slots = user_info.all_slots_by_id().await;
    params.insert(SLOTS_PARAM_KEY, all_slots.slot_info);
    params.insert(SELECTED_SLOT_PARAM_KEY, Typed::Int(user_info.selected_garage().await.1 as _));
    params.insert(SLOT_ORDER_PARAM_KEY, all_slots.slot_order);
    Ok(params.into())
}

pub struct GarageSlotsProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotsProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotsProvider {
    fn op_code() -> u8 {
        CODE
    }
}
