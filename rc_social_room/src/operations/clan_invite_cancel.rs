use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 45;

const PUBLIC_ID_PARAM_KEY: u8 = 1; // str; in

pub(super) struct ClanInviteCanceller {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanInviteCanceller {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(invitee)) = params.remove(&PUBLIC_ID_PARAM_KEY) {
            let user_info = user.user()?;
            log::debug!("User {} wants to cancel invite to user {}", user_info.public_id(), invitee.string);
            let (clan_info, members_info) = user_info.cancel_invite_to_clan(&invitee.string).await?;
            let mut online_clan_members: std::collections::HashSet<String> = members_info.iter()
                .map(|mem| mem.public_id.clone())
                .collect();
            self.social.filter_online_only(&mut online_clan_members).await;

            let event = crate::events::clan_invite_cancelled::ClanInviteCancelled {
                clan_name: clan_info.name,
            };
            self.social.send_event_to(&invitee.string, event).await;

            let my_pub_id = user_info.public_id();
            let event = crate::events::clan_member_left::ClanMemberLeft {
                leaver_public_id: my_pub_id.to_owned(),
                new_leader_public_id: None,
            };
            for online_member in online_clan_members.iter() {
                if online_member == my_pub_id { continue; }
                self.social.send_event_to(online_member, event.clone()).await;
            }
        }
        Ok(params)
    }
}

pub(super) fn clan_cancel_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanInviteCanceller> {
    SimpleOpImpl::new(ClanInviteCanceller {
        social: init_ctx.social.clone()
    })
}
