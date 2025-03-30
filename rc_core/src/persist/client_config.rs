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

impl std::convert::Into<crate::data::client_config::GameplaySettings> for GameplaySettings {
    fn into(self) -> crate::data::client_config::GameplaySettings {
        crate::data::client_config::GameplaySettings {
            show_tutorial_after_date: self.show_tutorial_after_date,
            health_threshold: self.health_threshold,
            microbot_sphere: self.microbot_sphere,
            misfire_angle: self.misfire_angle,
            shield_dps: self.shield_dps,
            shield_hps: self.shield_hps,
            request_review_level: self.request_review_level,
            critical_ratio: self.critical_ratio,
            cross_promo_image: self.cross_promo_image,
            cross_promo_link: self.cross_promo_link,
        }
    }
}
