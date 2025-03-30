use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default = "default_gameplay_settings")]
    pub gameplay: super::GameplaySettings,
    #[serde(default = "default_dev_messages")]
    pub banners: Vec<BannerMessage>,
}

fn default_gameplay_settings() -> super::GameplaySettings {
    super::GameplaySettings {
        show_tutorial_after_date: "2030-01-01".to_owned(),
        health_threshold: 0.20,
        microbot_sphere: 10.0,
        misfire_angle: 10.0,
        shield_dps: 100,
        shield_hps: 2_000,
        request_review_level: 10_000,
        critical_ratio: 5.0,
        cross_promo_image: "https://git.ngram.ca/assets/img/logo.png".to_owned(),
        cross_promo_link: "https://git.ngram.ca/OpenJam/servers".to_owned(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BannerMessage {
    pub message: String,
    pub duration: u32, // seconds
}

fn default_dev_messages() -> Vec<BannerMessage> {
    Vec::default()
}
