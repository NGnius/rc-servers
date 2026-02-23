use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan::*;

const CODE: u8 = 36;

const CLAN_NAME_PARAM_KEY: u8 = 31; // str; in and out
const CLAN_DESCRIPTION_PARAM_KEY: u8 = 32; // str; out
const CLAN_TYPE_PARAM_KEY: u8 = 34; // int; out
const ROBITS_CONVERSION_PARAM_KEY: u8 = 51; // out only
const MEMBERS_PARAM_KEY: u8 = 36; // hashmap (ClanMember); out

pub(super) struct ClanInviteAccepter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanInviteAccepter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(clan_name)) = params.remove(&CLAN_NAME_PARAM_KEY) {
            let user_info = user.user()?;
            log::debug!("User {} wants to accept invite to clan {}", user_info.public_id(), clan_name.string);
            let (clan_info, members_info) = user_info.join_clan(&clan_name.string).await?;
            let mut online_clan_members: std::collections::HashSet<String> = members_info.iter()
                .map(|mem| mem.public_id.clone())
                .collect();
            self.social.filter_online_only(&mut online_clan_members).await;
            log::debug!("Found clan info for {} ({} members, {} online)", clan_info.name, members_info.len(), online_clan_members.len());

            let my_pub_id = user_info.public_id();
            let self_member = members_info.iter()
                .find(|mem| mem.public_id == my_pub_id)
                .unwrap();
            let event = crate::events::clan_member_data_changed::ClanMemberDataUpdated {
                member_public_id: my_pub_id.to_owned(),
                member_display_name: user_info.display_name().to_owned(),
                avatar_id: self_member.avatar_id,
                state: Some(crate::data::clan::ClanMemberState::Confirmed),
                rank: None,
                is_online: Some(true),
            };
            for online_member in online_clan_members.iter() {
                if online_member == my_pub_id { continue; }
                self.social.send_event_to(online_member, event.clone()).await;
            }

            params.insert(MEMBERS_PARAM_KEY, Typed::Arr(Arr {
                ty: polariton::serdes::TypePrefix::HashMap, // hashmap
                custom_ty: None,
                items: members_info.into_iter()
                    .map(|member| ClanMember {
                        is_online: online_clan_members.contains(&member.public_id),
                        username: member.public_id,
                        display_name: member.display_name,
                        member_state: if member.is_confirmed { ClanMemberState::Confirmed } else { ClanMemberState::Invited },
                        use_custom_avatar: member.avatar_id.is_none(),
                        avatar_id: member.avatar_id.unwrap_or_default(),
                        rank: ClanMemberRank::from_core(member.rank),
                        season_xp: member.season_xp,
                    }.as_transmissible())
                    .collect(),
            }));
            params.insert(ROBITS_CONVERSION_PARAM_KEY, Typed::Float(0.5)); // TODO
            params.insert(CLAN_DESCRIPTION_PARAM_KEY, Typed::Str(clan_info.description.into()));
            params.insert(CLAN_NAME_PARAM_KEY, Typed::Str(clan_info.name.into()));
            params.insert(CLAN_TYPE_PARAM_KEY, Typed::Int(ClanType::from_core(clan_info.ty).to_u8() as _));
        }
        Ok(params)
    }
}

pub(super) fn clan_accept_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanInviteAccepter> {
    SimpleOpImpl::new(ClanInviteAccepter {
        social: init_ctx.social.clone()
    })
}
