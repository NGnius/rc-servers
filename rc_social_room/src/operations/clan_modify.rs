use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use crate::data::clan::*;

const CODE: u8 = 42;

const CLAN_DESC_PARAM_KEY: u8 = 32; // str; in
const CLAN_TYPE_PARAM_KEY: u8 = 34; // int (ClanType); in
const CLAN_AVATAR_PARAM_KEY: u8 = 33; // bytes; in

pub(super) struct ClanDataChanger {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanDataChanger {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let new_ty = if let Some(Typed::Int(clan_ty)) = params.remove(&CLAN_TYPE_PARAM_KEY) {
            let ty = ClanType::from_u8(clan_ty as u8)
                .ok_or_else(|| SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                    format!("Invalid clan type {}", clan_ty),
                ))?;
            Some(ty)
        } else {
            None
        };
        let new_clavatar = if let Some(Typed::Bytes(clavatar)) = params.remove(&CLAN_AVATAR_PARAM_KEY) {
            if clavatar.vec.is_empty() {
                None
            } else {
                Some(clavatar.vec)
            }
        } else {
            None
        };
        let new_desc = if let Some(Typed::Str(clan_desc)) = params.remove(&CLAN_DESC_PARAM_KEY) {
            // (technically this is a client bug)
            // when the game client updates a different field, the description will get sent as a zero-length string
            // which is impossible to differentiate from someone saving the description as zero-length
            // null/non-existent *should* be different than an empty string
            // So, let's try the closest guess the server can achieve:
            //  => assume a zero-length string means null *unless* all the other fields are null
            if clan_desc.string.is_empty() && (new_ty.is_some() || new_clavatar.is_some()) {
                None
            } else {
                Some(clan_desc.string)
            }
        } else {
            None
        };
        if new_desc.is_none() && new_ty.is_none() && new_clavatar.is_none() {
            log::debug!("Got clan data change no-op (doing nothing)");
            return Ok(params);
        }
        let user_info = user.user()?;
        let members = user_info.update_clan(
            None,
            new_desc.clone(),
            new_ty.map(|x| x.to_core()),
            new_clavatar,
        ).await?;
        let mut online_members = members.into_iter().map(|mem| mem.public_id).collect();
        self.social.filter_online_only(&mut online_members).await;
        let my_pub_id = user_info.public_id();
        let event = crate::events::clan_data_changed::ClanDataUpdated {
            description: new_desc,
            ty: new_ty,
        };
        for member in online_members {
            if member == my_pub_id { continue; }
            self.social.send_event_to(&member, event.clone()).await;
        }
        Ok(params)
    }
}

pub(super) fn update_clan_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanDataChanger> {
    SimpleOpImpl::new(ClanDataChanger {
        social: init_ctx.social.clone(),
    })
}
