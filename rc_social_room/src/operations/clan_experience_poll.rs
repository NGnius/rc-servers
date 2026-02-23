use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Dict};

const CODE: u8 = 57;

const EXPERIENCE_TABLE_PARAM_KEY: u8 = 48; // dict string -> int; out
const CLAN_NAME_PARAM_KEY: u8 = 31; // str; in

pub(super) struct ClanExperiencePoller;

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanExperiencePoller {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, _user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(clan)) = params.remove(&CLAN_NAME_PARAM_KEY) {
            log::debug!("Not providing clan experience for clan {} (not implemented)", clan.string);
            params.insert(EXPERIENCE_TABLE_PARAM_KEY, Typed::Dict(Dict {
                key_ty: polariton::serdes::TypePrefix::Str,
                val_ty: polariton::serdes::TypePrefix::Int,
                items: vec![
                    (Typed::Str("NGniusness".into()), Typed::Int(i32::MAX)),
                ],
            }));
        }
        Ok(params)
    }
}

pub(super) fn clan_experience_provider() -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanExperiencePoller> {
    SimpleOpImpl::new(ClanExperiencePoller)
}
