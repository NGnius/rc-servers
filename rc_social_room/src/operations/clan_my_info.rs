use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use crate::data::clan::*;

const CODE: u8 = 43;

const CLAN_NAME_PARAM_KEY: u8 = 31; // in and out
const CLAN_DESCRIPTION_PARAM_KEY: u8 = 32; // out only
const CLAN_TYPE_PARAM_KEY: u8 = 34; // out only

pub(super) struct MyClanInfoGetter {}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for MyClanInfoGetter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let clan_info = user_info.my_clan_info(false).await?;
        if let Some((clan_info, _members_info)) = clan_info {
            log::debug!("Found my clan info for {}", clan_info.name);
            params.insert(CLAN_DESCRIPTION_PARAM_KEY, Typed::Str(clan_info.description.into()));
            params.insert(CLAN_NAME_PARAM_KEY, Typed::Str(clan_info.name.into()));
            params.insert(CLAN_TYPE_PARAM_KEY, Typed::Int(ClanType::from_core(clan_info.ty).to_u8() as _));
        }
        Ok(params)
    }
}

pub(super) fn clan_info_provider() -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, MyClanInfoGetter> {
    SimpleOpImpl::new(MyClanInfoGetter {})
}
