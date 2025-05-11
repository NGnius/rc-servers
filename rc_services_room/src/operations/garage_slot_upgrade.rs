use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 39;

const CPU_INCREMENT_PARAM_KEY: u8 = 7;
const SUCCESS_PARAM_KEY: u8 = 39;

pub(super) fn garage_slot_upgrage_provider() -> GarageSlotUpgrade {
    GarageSlotUpgrade
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Int(increments)) = params.remove(&CPU_INCREMENT_PARAM_KEY) {
        let user_info = user.user()?;
        params.insert(SUCCESS_PARAM_KEY, user_info.upgrade_slot(increments).await?);
    }
    Ok(params.into())
}

pub struct GarageSlotUpgrade;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotUpgrade {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotUpgrade {
    fn op_code() -> u8 {
        CODE
    }
}
