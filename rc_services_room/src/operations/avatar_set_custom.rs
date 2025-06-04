use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 112;

const IMG_PARAM_KEY: u8 = 131; // int; in
const FORMAT_PARAM_KEY: u8 = 132; // int enum; in

pub(super) fn custom_avatar_upload_handler() -> CustomAvatarHandler {
    CustomAvatarHandler
}

async fn do_save(params: ParameterTable<()>, _user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    //let user_info = user.user()?;
    if let Some(Typed::Bytes(image)) = params.remove(&IMG_PARAM_KEY) {
        if let Some(Typed::Int(format)) = params.remove(&FORMAT_PARAM_KEY) {
            log::debug!("Got custom avatar ({}B) with format {}", image.vec.len(), format);
            // TODO actually save image
            /*let info = oj_rc_core::persist::user::AvatarInfo {
                avatar_id: 0,
                use_custom: true,
            };
            user_info.set_avatar_info(info).await?;*/
        }
    }
    Ok(params.into())
}

pub(super) struct CustomAvatarHandler;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for CustomAvatarHandler {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_save(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for CustomAvatarHandler {
    fn op_code() -> u8 {
        CODE
    }
}
