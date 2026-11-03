use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use oj_rc_core::types::RawType;

const CODE: u8 = 1;

const USERNAME_PARAM_KEY: u8 = 1; // str; in & out
const IS_ONLINE_PARAM_KEY: u8 = 2; // bool; out

pub(super) struct FriendRequestAccepter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for FriendRequestAccepter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            let user_info = user.user()?;
            let user_pub_id = oj_rc_core::types::PublicId::from_raw(username.string);
            user_info.accept_friend(&user_pub_id).await?;
            self.social.send_event_to(&user_pub_id, crate::events::friend_invite_accepted::FriendInviteAccepted {
                friend_public_id: user_info.public_id(),
                friend_display_name: user_info.display_name(),
            }).await;
            params.insert(IS_ONLINE_PARAM_KEY, Typed::Bool(true)); // when would this ever not be true??
        }
        Ok(params)
    }
}

pub(super) fn friend_accept_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, FriendRequestAccepter> {
    SimpleOpImpl::new(FriendRequestAccepter {
        social: init_ctx.social.clone(),
    })
}
