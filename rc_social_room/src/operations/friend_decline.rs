use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 2;

const USERNAME_PARAM_KEY: u8 = 1; // str; in & out

pub(super) struct FriendRequestDecliner {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for FriendRequestDecliner {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            let user_info = user.user()?;
            user_info.decline_friend(username.string.clone()).await?;
            self.social.send_event_to(&username.string, crate::events::friend_invite_declined::FriendInviteDeclined {
                friend_public_id: user_info.public_id().to_owned(),
                friend_display_name: user_info.display_name().to_owned(),
            }).await;
        }
        Ok(params)
    }
}

pub(super) fn friend_decline_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, FriendRequestDecliner> {
    SimpleOpImpl::new(FriendRequestDecliner {
        social: init_ctx.social.clone(),
    })
}
