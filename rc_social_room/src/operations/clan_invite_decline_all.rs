use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 44;

pub(super) struct ClanInviteDeclineAller {
    //social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanInviteDeclineAller {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        log::debug!("User {} wants to decline all clan invites", user_info.public_id());
        user_info.decline_all_clan_invites().await?;
        // TODO send clan member leave events to all clans
        Ok(params)
    }
}

pub(super) fn clan_decline_all_provider(_init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanInviteDeclineAller> {
    SimpleOpImpl::new(ClanInviteDeclineAller {
        //social: init_ctx.social.clone()
    })
}
