use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

//use crate::data::friend::*;

const CODE: u8 = 19;

const INVITER_NAME_PARAM_KEY: u8 = 19;
const INVITER_DISPLAY_NAME_PARAM_KEY: u8 = 75;
const INVITER_CUSTOM_AVATAR_NAME_PARAM_KEY: u8 = 13;
const INVITER_AVATAR_ID_NAME_PARAM_KEY: u8 = 14;

pub(super) struct PlatoonPendingInviter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonPendingInviter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let my_public_id = user_info.public_id();
        log::debug!("Checking for pending platoon invite of user {}", my_public_id);
        if let Some(platoon_id) = self.social.platoon_invite_of_user(my_public_id).await {
            let members = self.social.users_of_platoon(&platoon_id).await;
            let social_infos = user_info.list_social_info(&[members[0].public_id.clone()]).await?;
            params.insert(INVITER_NAME_PARAM_KEY, Typed::Str(social_infos[0].public_id.clone().into()));
            params.insert(INVITER_DISPLAY_NAME_PARAM_KEY, Typed::Str(social_infos[0].display_name.clone().into()));
            params.insert(INVITER_CUSTOM_AVATAR_NAME_PARAM_KEY, Typed::Bool(social_infos[0].avatar_id.is_none()));
            params.insert(INVITER_AVATAR_ID_NAME_PARAM_KEY, Typed::Int(social_infos[0].avatar_id.unwrap_or_default()));
        }
        Ok(params)
    }
}

pub(super) fn platoon_pending_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonPendingInviter> {
    SimpleOpImpl::new(PlatoonPendingInviter {
        social: init_ctx.social.clone(),
    })
}
