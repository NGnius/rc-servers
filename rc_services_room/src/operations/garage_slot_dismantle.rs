use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 42;

const SLOT_PARAM_KEY: u8 = 43;

pub(super) fn garage_slot_dismantler() -> GarageSlotDismantler {
    GarageSlotDismantler
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Int(slot)) = params.remove(&SLOT_PARAM_KEY) {
        let user_info = user.user()?;
        user_info.new_slot(Some(slot)).await?;
    }
    Ok(params.into())
}

pub struct GarageSlotDismantler;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotDismantler {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotDismantler {
    fn op_code() -> u8 {
        CODE
    }
}
