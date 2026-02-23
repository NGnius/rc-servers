use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 40;

pub(super) struct ClanLeaver {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanLeaver {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let mut members = if let Some((_clan, members)) = user_info.my_clan_info(true).await? {
            members.into_iter().map(|mem| mem.public_id).collect()
        } else {
            std::collections::HashSet::default()
        };
        log::debug!("User {} wants to leave their clan", user_info.public_id());
        user_info.leave_clan().await?;
        self.social.filter_online_only(&mut members).await;
        let my_pub_id = user_info.public_id();
        let event = crate::events::clan_member_left::ClanMemberLeft {
            leaver_public_id: my_pub_id.to_owned(),
            new_leader_public_id: None,
        };
        for member in members {
            if member == my_pub_id { continue; }
            self.social.send_event_to(&member, event.clone()).await;
        }
        Ok(params)
    }
}

pub(super) fn clan_leave_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanLeaver> {
    SimpleOpImpl::new(ClanLeaver {
        social: init_ctx.social.clone(),
    })
}
