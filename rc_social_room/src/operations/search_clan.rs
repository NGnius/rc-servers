use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan::*;

const CODE: u8 = 32;

// params in
const STRING_PARAM_KEY: u8 = 39;
const DAYS_SINCE_ACTIVE_PARAM_KEY: u8 = 40;
const START_RANGE_PARAM_KEY: u8 = 41;
const END_RANGE_PARAM_KEY: u8 = 43;
const TYPES_PARAM_KEY: u8 = 34;

// params out
const RESULTS_PARAM_KEY: u8 = 42;

/*pub(super) fn search_clans_provider<C: Send + Sync>() -> SimpleFunc<32, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(s)) = params.get(&STRING_PARAM_KEY) {
            if !s.string.is_empty() {
                log::debug!("Got clan search string `{}`", s.string);
            }
        }
        params.insert(RESULTS_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            custom_ty: None,
            items: vec![
                ClanInfo {
                    clan_name: "RE_clan_search_name".to_owned(),
                    clan_description: "RE_clan_search_description".to_owned(),
                    clan_type: ClanType::Closed,
                    clan_size: 1,
                }.as_transmissible(),
            ],
        }));
        Ok(params.into())
    })
}*/

pub(super) struct ClansSearcher {}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClansSearcher {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let search_string = if let Some(Typed::Str(search)) = params.remove(&STRING_PARAM_KEY) {
            search.string
        } else {
            String::default()
        };
        if let Some(Typed::Int(days_since_active)) = params.remove(&DAYS_SINCE_ACTIVE_PARAM_KEY) {
            if let Some(Typed::Int(start_range)) = params.remove(&START_RANGE_PARAM_KEY) {
                if let Some(Typed::Int(end_range)) = params.remove(&END_RANGE_PARAM_KEY) {
                    if let Some(Typed::Arr(clan_types)) = params.remove(&TYPES_PARAM_KEY) {
                        let mut types = Vec::with_capacity(clan_types.items.len());
                        for item in clan_types.items {
                            if let Typed::Int(ty) = item {
                                types.push(
                                    crate::data::clan::ClanType::from_u8(ty as u8)
                                        .ok_or_else(|| SimpleOpError::with_message(
                                            oj_rc_core::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                                            format!("Invalid clan type {}", ty),
                                        ))?
                                        .to_core()
                                );
                            }
                        }
                        let search_query = oj_rc_core::persist::user::ClanSearchQuery {
                            search_string,
                            days_since_active,
                            start_range,
                            end_range,
                            types,
                        };
                        let user_info = user.user()?;
                        log::debug!("Searching for clans with query {:?}", search_query);
                        let results = user_info.search_clan(search_query).await?;
                        log::debug!("Got {} clans from search", results.len());
                        params.insert(RESULTS_PARAM_KEY, Typed::Arr(Arr {
                            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
                            custom_ty: None,
                            items: results.into_iter()
                                .map(|clan| ClanInfo {
                                    clan_name: clan.name,
                                    clan_description: clan.description,
                                    clan_type: ClanType::from_core(clan.ty),
                                    clan_size: clan.size,
                                }.as_transmissible())
                                .collect(),
                        }));
                    }
                }
            }
        }
        Ok(params)
    }
}

pub(super) fn search_clans_provider() -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClansSearcher> {
    SimpleOpImpl::new(ClansSearcher {})
}
