use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameplaySettings {
    pub show_tutorial_after_date: String,
    pub health_threshold: f32, // percent
    pub microbot_sphere: f32, // radius
    pub misfire_angle: f32, // degrees?
    pub shield_dps: i32,
    pub shield_hps: u32,
    pub request_review_level: u32,
    pub critical_ratio: f32,
    pub cross_promo_image: String, // url
    pub cross_promo_link: String, // url
}

impl std::convert::From<GameplaySettings> for crate::data::client_config::GameplaySettings {
    fn from(val: GameplaySettings) -> Self {
        crate::data::client_config::GameplaySettings {
            show_tutorial_after_date: val.show_tutorial_after_date,
            health_threshold: val.health_threshold,
            microbot_sphere: val.microbot_sphere,
            misfire_angle: val.misfire_angle,
            shield_dps: val.shield_dps,
            shield_hps: val.shield_hps,
            request_review_level: val.request_review_level,
            critical_ratio: val.critical_ratio,
            cross_promo_image: val.cross_promo_image,
            cross_promo_link: val.cross_promo_link,
        }
    }
}
