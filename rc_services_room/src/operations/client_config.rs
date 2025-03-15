use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::client_config::*;

const PARAM_KEY: u8 = 36;

pub(super) fn client_config_provider() -> SimpleFunc<34, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashtable
            items: vec![
                (Typed::Str("GameplaySettings".into()), GameplaySettings {
                    show_tutorial_after_date: "2025-01-01".to_owned(),
                    health_threshold: 10.0,
                    microbot_sphere: 10.0,
                    misfire_angle: 20.0,
                    shield_dps: 100,
                    shield_hps: 2_000,
                    request_review_level: 10_000,
                    critical_ratio: 10.0,
                    cross_promo_image: "https://git.ngram.ca/assets/img/logo.png".to_owned(),
                    cross_promo_link: "https://git.ngram.ca/OpenJam/servers".to_owned(),
                }.as_transmissible())
            ].into(),
        }));
        Ok(params.into())
    })
}
