use polariton_server::operations::{Immediate, SimpleFunc};
use polariton::operation::{ParameterTable, Typed};

use crate::persist::config::ConfigProvider;

const CAMPAIGNS_BYTES_PARAM_KEY: u8 = 64; // list of bytes (serialised data)
const CAMPAIGNS_WAVES_PARAM_KEY: u8 = 70; // hashtable
const CAMPAIGNS_VERSIONS_PARAM_KEY: u8 = 69; // hashtable

pub(super) fn singleplayer_campaigns_provider(conf: &crate::persist::config::ConfigImpl) -> Immediate<65, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::new();
        params.insert(CAMPAIGNS_BYTES_PARAM_KEY, conf.campaigns_parameters()); // first 4 bytes are i32 for length of the rest
        //params.insert(CAMPAIGNS_BYTES_PARAM_KEY, Typed::Bytes(vec![0u8, 0u8, 0u8, 0u8].into())); // first 4 bytes are i32 for length of the rest
        params.insert(CAMPAIGNS_WAVES_PARAM_KEY, conf.campaign_waves());
        //params.insert(CAMPAIGNS_WAVES_PARAM_KEY, Typed::HashMap(vec![].into()));
        params.insert(CAMPAIGNS_VERSIONS_PARAM_KEY, conf.campaign_version());
        // params.insert(CAMPAIGNS_VERSIONS_PARAM_KEY, Typed::HashMap(vec![
        //     (Typed::Str("CurrentVersionNumber".into()), Typed::Int(0)),
        //     (Typed::Str("LockedCampaignsInfo".into()), Typed::HashMap(vec![
        //         (Typed::Str("0".into()), Typed::Bool(false.into()))
        //     ].into())),
        // ].into()));
        params.into()
    })
}

const CAMPAIGN_ID_PARAM_KEY: u8 = 22; // string; in
const CAMPAIGN_DIFFICULTY_PARAM_KEY: u8 = 23; // i32; in
const CAMPAIGN_WAVES_PARAM_KEY: u8 = 75; // bytes; out

pub(super) fn singleplayer_complete_campaign_provider(conf: &crate::persist::config::ConfigImpl) -> SimpleFunc<64, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    let campaign_details = <crate::persist::config::ConfigImpl as crate::persist::config::ConfigProvider<()>>::campaign_details(conf);
    SimpleFunc::new(move |params, _| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(campaign_id)) = params.get(&CAMPAIGN_ID_PARAM_KEY) {
            if let Some(Typed::Int(campaign_difficulty)) = params.get(&CAMPAIGN_DIFFICULTY_PARAM_KEY) {
                let complete_campaign = campaign_details.get(&campaign_id.string, campaign_difficulty)?;
                params.clear();
                params.insert(CAMPAIGN_WAVES_PARAM_KEY, complete_campaign);
            }
        }
        Ok(params.into())
    })
}

const CAMPAIGN_WAVE_NUMBER_PARAM_KEYL: u8 = 73;

pub(super) fn singleplayer_save_complete_campaign_provider() -> SimpleFunc<68, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    //let campaign_details = <crate::persist::config::ConfigImpl as crate::persist::config::ConfigProvider<()>>::campaign_details(conf);
    SimpleFunc::new(move |params, user: &crate::UserTy| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(campaign_id)) = params.get(&CAMPAIGN_ID_PARAM_KEY) {
            if let Some(Typed::Int(campaign_difficulty)) = params.get(&CAMPAIGN_DIFFICULTY_PARAM_KEY) {
                if let Some(Typed::Int(wave_number)) = params.get(&CAMPAIGN_WAVE_NUMBER_PARAM_KEYL) {
                    let user_lock = user.read().unwrap();
                    log::info!("User {} completed campaign {} difficulty {} wave {}", user_lock.user()?.token().uuid, campaign_id.string, campaign_difficulty, wave_number);
                    // TODO save wave as completed
                    params.clear();
                }
            }
        }
        Ok(params.into())
    })
}
