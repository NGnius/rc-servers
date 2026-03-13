use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 58;

const USERNAME_PARAM_KEY: u8 = 65; // str; in
const RESPONSE_CODE_PARAM_KEY: u8 = 66; // bool; out

pub(super) struct UserCanBeInvitedToCustoGameGetter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for UserCanBeInvitedToCustoGameGetter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            let _user_info = user.user()?; // just to validate request is authenticated
            let mut set = std::collections::HashSet::with_capacity(1);
            set.insert(username.string.clone());
            self.social.filter_online_only(&mut set).await;
            if set.is_empty() {
                params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Bool(false));
                return Ok(params);
            }
            if self.social.platoon_of_user(&username.string).await.is_some() {
                params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Bool(false));
                return Ok(params);
            }
            params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Bool(true));
        }
        Ok(params)
    }
}

pub(super) fn can_invite_to_custom_game_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, UserCanBeInvitedToCustoGameGetter> {
    SimpleOpImpl::new(UserCanBeInvitedToCustoGameGetter {
        social: init_ctx.social.clone(),
    })
}
