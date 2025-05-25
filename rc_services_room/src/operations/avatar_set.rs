use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 111;

const IS_CUSTOM_PARAM_KEY: u8 = 130; // bool; in
const AVATAR_ID_PARAM_KEY: u8 = 129; // int; in

pub(super) fn avatar_set_provider() -> AvatarSetProvider {
    AvatarSetProvider
}

async fn do_save(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    let user_info = user.user()?;
    if let Some(Typed::Int(avatar)) = params.remove(&AVATAR_ID_PARAM_KEY) {
        if let Some(Typed::Bool(is_custom)) = params.remove(&IS_CUSTOM_PARAM_KEY) {
            let info = rc_core::persist::user::AvatarInfo {
                avatar_id: avatar,
                use_custom: is_custom,
            };
            user_info.set_avatar_info(info).await?;
        }
    }
    Ok(params.into())
}

pub(super) struct AvatarSetProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for AvatarSetProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_save(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for AvatarSetProvider {
    fn op_code() -> u8 {
        CODE
    }
}
