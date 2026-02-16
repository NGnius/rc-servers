use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 0;

const USERNAME_PARAM_KEY: u8 = 1; // str; in & out
const DISPLAY_NAME_PARAM_KEY: u8 = 75; // str; out
const CLAN_NAME_PARAM_KEY: u8 = 31; // str; out TODO
const USER_DATA_PARAM_KEY: u8 = 9; // hashtable; out

pub(super) struct FriendRequestMaker {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for FriendRequestMaker {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            let user_info = user.user()?;
            let resp = user_info.invite_friend(username.string).await?;
            self.social.send_event_to(&resp.target_public_id, crate::events::friend_invite_received::FriendInviteReceived {
                friend_public_id: user_info.public_id().to_owned(),
                friend_display_name: user_info.display_name().to_owned(),
                clan_name: resp.my_clan_name,
                is_online: true,
                avatar_id: resp.my_avatar_id,
            }).await;
            params.insert(USERNAME_PARAM_KEY, Typed::Str(resp.target_public_id.into()));
            params.insert(DISPLAY_NAME_PARAM_KEY, Typed::Str(resp.target_display_name.into()));
            if let Some(clan_name) = resp.target_clan_name {
                params.insert(CLAN_NAME_PARAM_KEY, Typed::Str(clan_name.into()));
            }
            params.insert(USER_DATA_PARAM_KEY, resp.target_player);
        }
        Ok(params)
    }
}

pub(super) fn friend_invite_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, FriendRequestMaker> {
    SimpleOpImpl::new(FriendRequestMaker {
        social: init_ctx.social.clone(),
    })
}
