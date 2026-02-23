use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 35;

const PUBLIC_ID_PARAM_KEY: u8 = 1; // str; in
const DISPLAY_NAME_PARAM_KEY: u8 = 75; // str; out
const CUSTOM_AVATAR_PARAM_KEY: u8 = 13; // bool; out
const AVATAR_ID_PARAM_KEY: u8 = 14; // int; out
const SEASON_XP_PARAM_KEY: u8 = 48; // int; out

pub(super) struct ClanInviter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanInviter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(invitee)) = params.remove(&PUBLIC_ID_PARAM_KEY) {
            let user_info = user.user()?;
            log::debug!("User {} wants to invite {} to their clan", user_info.account_id(), invitee.string);
            let invitee = user_info.invite_to_clan(&invitee.string).await?;
            params.insert(PUBLIC_ID_PARAM_KEY, Typed::Str(invitee.public_id.clone().into()));
            params.insert(DISPLAY_NAME_PARAM_KEY, Typed::Str(invitee.display_name.clone().into()));
            params.insert(CUSTOM_AVATAR_PARAM_KEY, Typed::Bool(invitee.avatar_id.is_none()));
            params.insert(AVATAR_ID_PARAM_KEY, Typed::Int(invitee.avatar_id.unwrap_or_default()));
            params.insert(SEASON_XP_PARAM_KEY, Typed::Int(invitee.season_xp));
            if let Some((my_clan, my_clan_members)) = user_info.my_clan_info(true).await? {
                let my_pub_id = user_info.public_id();
                let my_soc_info = user_info.list_social_info(&[my_pub_id.to_owned()]).await?;
                let event = crate::events::clan_invite_received::ClanInviteReceived {
                    inviter_public_id: my_pub_id.to_owned(),
                    inviter_display_name: user_info.display_name().to_owned(),
                    clan_size: my_clan.size,
                    clan_name: my_clan.name,
                    avatar_id: my_soc_info.first().and_then(|x| x.avatar_id),
                };
                self.social.send_event_to(&invitee.public_id, event).await;

                let event = crate::events::clan_member_joined::ClanMemberJoined {
                    joiner_public_id: invitee.public_id,
                    joiner_display_name: invitee.display_name,
                    avatar_id: invitee.avatar_id,
                    state: crate::data::clan::ClanMemberState::Invited,
                    season_xp: invitee.season_xp,
                };
                for member in my_clan_members {
                    if member.public_id == my_pub_id { continue; }
                    self.social.send_event_to(&member.public_id, event.clone()).await;
                }
            }
        }
        Ok(params)
    }
}

pub(super) fn invite_to_clan_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanInviter> {
    SimpleOpImpl::new(ClanInviter {
        social: init_ctx.social.clone(),
    })
}
