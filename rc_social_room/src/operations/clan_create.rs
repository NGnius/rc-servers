use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan::*;

const CODE: u8 = 31;

const CLAN_NAME_PARAM_KEY: u8 = 31; // str; in
const CLAN_DESC_PARAM_KEY: u8 = 32; // str; in
const CLAN_TYPE_PARAM_KEY: u8 = 34; // int (ClanType); in
const CLAN_AVATAR_PARAM_KEY: u8 = 33; // bytes; in
const MEMBERS_PARAM_KEY: u8 = 36; // arr of hashmap (ClanMember); out

pub(super) struct ClanCreator {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanCreator {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(clan_name)) = params.remove(&CLAN_NAME_PARAM_KEY) {
            if let Some(Typed::Str(clan_desc)) = params.remove(&CLAN_DESC_PARAM_KEY) {
                if let Some(Typed::Int(clan_ty)) = params.remove(&CLAN_TYPE_PARAM_KEY) {
                    let ty = ClanType::from_u8(clan_ty as u8)
                        .ok_or_else(|| SimpleOpError::with_message(
                            oj_rc_core::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                            format!("Invalid clan type {}", clan_ty),
                        ))?;
                    let clavatar = if let Some(Typed::Bytes(clavatar)) = params.remove(&CLAN_AVATAR_PARAM_KEY) {
                        clavatar.vec
                    } else {
                        Vec::default()
                    };
                    let new_clan = oj_rc_core::persist::user::ClanData {
                        name: clan_name.string,
                        description: clan_desc.string,
                        ty: ty.to_core(),
                        size: 0,
                    };
                    let user_info = user.user()?;
                    let new_members = user_info.create_clan(new_clan, clavatar).await?;
                    let mut online_clan_members: std::collections::HashSet<String> = new_members.iter()
                        .map(|mem| mem.public_id.clone())
                        .collect();
                    self.social.filter_online_only(&mut online_clan_members).await;
                    params.insert(MEMBERS_PARAM_KEY, Typed::Arr(Arr {
                        ty: polariton::serdes::TypePrefix::HashMap,
                        custom_ty: None,
                        items: new_members.into_iter()
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
                            .collect()
                    }));
                }
            }
        }
        Ok(params)
    }
}

pub(super) fn creat_clan_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanCreator> {
    SimpleOpImpl::new(ClanCreator {
        social: init_ctx.social.clone(),
    })
}
