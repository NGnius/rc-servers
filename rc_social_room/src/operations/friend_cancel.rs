use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use oj_rc_core::types::RawType;

const CODE: u8 = 5;

const USERNAME_PARAM_KEY: u8 = 1; // str; in & out

pub(super) struct FriendRequestCanceller {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for FriendRequestCanceller {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            let user_info = user.user()?;
            let user_pub_id = oj_rc_core::types::PublicId::from_raw(username.string);
            user_info.cancel_friend(&user_pub_id).await?;
            self.social.send_event_to(&user_pub_id, crate::events::friend_invite_cancelled::FriendInviteCancelled {
                friend_public_id: user_info.public_id(),
                friend_display_name: user_info.display_name(),
            }).await;
        }
        Ok(params)
    }
}

pub(super) fn friend_cancel_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, FriendRequestCanceller> {
    SimpleOpImpl::new(FriendRequestCanceller {
        social: init_ctx.social.clone(),
    })
}
