use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 41;

const USERNAME_PARAM_KEY: u8 = 1; // str; in

pub(super) struct ClanRemover {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanRemover {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(to_remove)) = params.remove(&USERNAME_PARAM_KEY) {
            let user_info = user.user()?;
            let mut members = if let Some((_clan, members)) = user_info.my_clan_info(true).await? {
                members.into_iter().map(|mem| mem.public_id).collect()
            } else {
                std::collections::HashSet::default()
            };
            log::debug!("User {} wants to remove {} from their clan", user_info.public_id(), to_remove.string);
            user_info.remove_user_from_clan(&to_remove.string).await?;
            self.social.send_event_to(&to_remove.string, crate::events::clan_member_removed::ClanMemberRemoved).await;
            self.social.filter_online_only(&mut members).await;
            let my_pub_id = user_info.public_id();
            let event = crate::events::clan_member_left::ClanMemberLeft {
                leaver_public_id: to_remove.string.clone(),
                new_leader_public_id: None,
            };
            for member in members {
                if member == to_remove.string { continue; }
                if member == my_pub_id { continue; }
                self.social.send_event_to(&member, event.clone()).await;
            }
        }
        Ok(params)
    }
}

pub(super) fn clan_remove_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanRemover> {
    SimpleOpImpl::new(ClanRemover {
        social: init_ctx.social.clone(),
    })
}
