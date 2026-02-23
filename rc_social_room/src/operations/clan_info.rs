use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan::*;

const CODE: u8 = 33;

const CLAN_NAME_PARAM_KEY: u8 = 31; // in and out
const MEMBERS_PARAM_KEY: u8 = 36; // out only
const ROBITS_CONVERSION_PARAM_KEY: u8 = 51; // out only
const CLAN_DESCRIPTION_PARAM_KEY: u8 = 32; // out only
const CLAN_TYPE_PARAM_KEY: u8 = 34; // out only

/*pub(super) fn clan_info_provider<C: Send + Sync>() -> SimpleFunc<33, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(clan_name)) = params.get(&CLAN_NAME_PARAM_KEY) {
            log::debug!("Requested info on clan {}", clan_name.string);
            params.insert(MEMBERS_PARAM_KEY, Typed::Arr(Arr {
                ty: polariton::serdes::TypePrefix::HashMap, // hashmap
                custom_ty: None,
                items: vec![
                    ClanMember {
                        username: "RE_clan_user_idk0".to_owned(),
                        display_name: "RE_clan_user_idk0".to_owned(),
                        member_state: ClanMemberState::Idk0,
                        use_custom_avatar: false,
                        avatar_id: 1,
                        rank: ClanMemberRank::Leader,
                        is_online: true,
                        season_xp: 42_000,
                    }.as_transmissible(),
                    ClanMember {
                        username: "RE_clan_user_idk1".to_owned(),
                        display_name: "RE_clan_user_idk1".to_owned(),
                        member_state: ClanMemberState::Idk1,
                        use_custom_avatar: false,
                        avatar_id: 1,
                        rank: ClanMemberRank::Leader,
                        is_online: true,
                        season_xp: 42_001,
                    }.as_transmissible(),
                    ClanMember {
                        username: "RE_clan_user_idk2".to_owned(),
                        display_name: "RE_clan_user_idk2".to_owned(),
                        member_state: ClanMemberState::Idk2,
                        use_custom_avatar: false,
                        avatar_id: 1,
                        rank: ClanMemberRank::Leader,
                        is_online: true,
                        season_xp: 42_002,
                    }.as_transmissible(),
                ],
            }));
            params.insert(ROBITS_CONVERSION_PARAM_KEY, Typed::Float(0.5));
            params.insert(CLAN_DESCRIPTION_PARAM_KEY, Typed::Str("RE_clan_description".into()));
            params.insert(CLAN_TYPE_PARAM_KEY, Typed::Int(ClanType::Closed as _));
        } else {
            log::debug!("Requested info on own clan (returning no info)");
        }

        Ok(params.into())
    })
}*/

pub(super) struct ClanInfoGetter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanInfoGetter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let clan_info = if let Some(Typed::Str(clan_name)) = params.remove(&CLAN_NAME_PARAM_KEY) {
            // retrieve info for other clan (return nothing if clan does not exist)
            log::debug!("Getting {} clan info", clan_name.string);
            user_info.clan_info(&clan_name.string).await?
        } else {
            // retrieve info for user's own clan (return nothing if user not a part of a clan)
            log::debug!("Getting own clan info");
            user_info.my_clan_info(true).await?
        };
        if let Some((clan_info, members_info)) = clan_info {
            let mut online_clan_members: std::collections::HashSet<String> = members_info.iter()
                .map(|mem| mem.public_id.clone())
                .collect();
            self.social.filter_online_only(&mut online_clan_members).await;
            log::debug!("Found clan info for {} ({} members, {} online)", clan_info.name, members_info.len(), online_clan_members.len());
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
            params.insert(ROBITS_CONVERSION_PARAM_KEY, Typed::Float(0.5)); // TODO ???
            params.insert(CLAN_DESCRIPTION_PARAM_KEY, Typed::Str(clan_info.description.into()));
            params.insert(CLAN_NAME_PARAM_KEY, Typed::Str(clan_info.name.into()));
            params.insert(CLAN_TYPE_PARAM_KEY, Typed::Int(ClanType::from_core(clan_info.ty).to_u8() as _));
        }
        Ok(params)
    }
}

pub(super) fn clan_info_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanInfoGetter> {
    SimpleOpImpl::new(ClanInfoGetter {
        social: init_ctx.social.clone(),
    })
}
