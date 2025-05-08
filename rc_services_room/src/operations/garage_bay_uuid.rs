use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 177;

const PARAM_KEY: u8 = 54;

pub(super) fn garage_id_provider() -> GarageIdProvider {
    GarageIdProvider
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let user_info = user.user()?;
    let mut params = params.to_dict();
    params.insert(PARAM_KEY, Typed::Str(user_info.selected_garage().await.0.into()));
    Ok(params.into())
}

pub struct GarageIdProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageIdProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageIdProvider {
    fn op_code() -> u8 {
        CODE
    }
}
