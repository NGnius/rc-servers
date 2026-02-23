use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 38;

const PUBLIC_ID_PARAM_KEY: u8 = 1; // str; in and out
const MEMBER_RANK_PARAM_KEY: u8 = 38; // int enum (ClanMemberRank); in

pub(super) struct ClanMemberRankChanger {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanMemberRankChanger {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(member)) = params.remove(&PUBLIC_ID_PARAM_KEY) {
            if let Some(Typed::Int(new_rank)) = params.remove(&MEMBER_RANK_PARAM_KEY) {
                let new_rank = crate::data::clan::ClanMemberRank::from_u8(new_rank as _)
                    .ok_or_else(|| SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                        format!("Invalid clan member rank {}", new_rank),
                    ))?;
                let user_info = user.user()?;
                let my_pub_id = user_info.public_id();
                log::debug!("User {} wants to update user {} to clan member rank {:?}", my_pub_id, member.string, new_rank);
                let members_info = user_info.update_clan_member(&member.string, new_rank.to_core()).await?;
                let mut online_clan_members: std::collections::HashSet<String> = members_info.iter()
                    .map(|mem| mem.public_id.clone())
                    .collect();
                self.social.filter_online_only(&mut online_clan_members).await;
                let target_member_opt = members_info.iter()
                    .find(|mem| mem.public_id == member.string);
                if let Some(target_member) = target_member_opt {
                    let event = crate::events::clan_member_data_changed::ClanMemberDataUpdated {
                        member_public_id: target_member.public_id.clone(),
                        member_display_name: target_member.display_name.clone(),
                        avatar_id: target_member.avatar_id,
                        state: None,
                        rank: Some(new_rank),
                        is_online: Some(true),
                    };
                    for online_member in online_clan_members {
                        if online_member == my_pub_id { continue; }
                        //if online_member == member.string { continue; }
                        self.social.send_event_to(&online_member, event.clone()).await;
                    }
                }
            }
        }
        Ok(params)
    }
}

pub(super) fn clan_rank_change_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanMemberRankChanger> {
    SimpleOpImpl::new(ClanMemberRankChanger {
        social: init_ctx.social.clone()
    })
}
