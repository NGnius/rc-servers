use polariton::operation::{ParameterTable, OperationResponse};

const CODE: u8 = 110;

const IS_CUSTOM_PARAM_KEY: u8 = 130; // bool
const AVATAR_ID_PARAM_KEY: u8 = 129; // int

pub(super) fn avatar_get_provider() -> AvatarGetProvider {
    AvatarGetProvider
}

async fn do_save(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    let user_info = user.user()?;
    let info = user_info.get_avatar_info().await?;
    params.insert(IS_CUSTOM_PARAM_KEY, info.use_custom);
    params.insert(AVATAR_ID_PARAM_KEY, info.avatar_id);
    Ok(params.into())
}

pub(super) struct AvatarGetProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for AvatarGetProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_save(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for AvatarGetProvider {
    fn op_code() -> u8 {
        CODE
    }
}
