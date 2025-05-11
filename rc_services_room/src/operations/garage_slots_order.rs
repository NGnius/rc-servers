use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 114;

const SLOT_ORDER_PARAM_KEY: u8 = 58;

pub(super) fn garage_slot_order_provider() -> GarageSlotsOrderProvider {
    GarageSlotsOrderProvider
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Arr(order)) = params.remove(&SLOT_ORDER_PARAM_KEY) {
        //log::info!("Slot order is {}", order);
        let mut order_i32 = Vec::with_capacity(order.items.len());
        for item in order.items.iter() {
            if let Typed::Int(item) = item {
                order_i32.push(*item);
            }
        }
        let user_info = user.user()?;
        user_info.save_slot_order(order_i32).await?;
    }
    Ok(params.into())
}

pub struct GarageSlotsOrderProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotsOrderProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotsOrderProvider {
    fn op_code() -> u8 {
        CODE
    }
}
