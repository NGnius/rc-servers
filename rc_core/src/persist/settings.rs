use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default = "default_gameplay_settings")]
    pub gameplay: super::GameplaySettings,
    #[serde(default = "default_dev_messages")]
    pub banners: Vec<BannerMessage>,
    #[serde(default = "default_slot_upgrades")]
    pub garage_upgrades: Vec<GarageSlotUpgrade>,
    #[serde(default = "default_server_conf")]
    pub server: ServerSettings,
}

fn default_gameplay_settings() -> super::GameplaySettings {
    super::GameplaySettings {
        show_tutorial_after_date: "2030-01-01".to_owned(),
        health_threshold: 0.20,
        microbot_sphere: 0.5,
        misfire_angle: 10.0,
        shield_dps: 100_000,
        shield_hps: 10_000_000,
        request_review_level: 10_000,
        critical_ratio: 5.0,
        cross_promo_image: "https://git.ngram.ca/OpenJam/servers/raw/branch/main/assets/robocraft/default.png".to_owned(),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GarageSlotUpgrade {
    pub cpu: u32,
    pub cost: u32,
}

fn default_slot_upgrades() -> Vec<GarageSlotUpgrade> {
    vec![
        GarageSlotUpgrade {
            cpu: 100,
            cost: 100,
        },
        GarageSlotUpgrade {
            cpu: 200,
            cost: 200,
        },
        GarageSlotUpgrade {
            cpu: 1_000,
            cost: 1_000,
        },
        GarageSlotUpgrade {
            cpu: 2_000,
            cost: 2_000,
        },
        GarageSlotUpgrade {
            cpu: 10_000,
            cost: 10_000,
        },
    ]
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerSettings {
    #[serde(default = "default_db_conn")]
    pub database: String,
    #[serde(default)]
    pub auto_signup: bool,
    #[serde(default)]
    pub queue_mode: QueueMode,
    #[serde(default = "default_cdn_root_url")]
    pub cdn_url: String,
    #[serde(default = "default_feedback_url")]
    pub feedback_url: String,
    #[serde(default = "default_support_url")]
    pub support_url: String,
    #[serde(default = "default_wiki_url")]
    pub wiki_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum QueueMode {
    Upgrade, // move enqueued players into newer gamemode
    #[default]
    Notify, // send match change
    Ignore, // do nothing
}

fn default_db_conn() -> String {
    "sqlite:../data/robocraft/accounts.sqlite.db?mode=rwc".to_owned()
}

fn default_server_conf() -> ServerSettings {
    ServerSettings {
        database: default_db_conn(),
        auto_signup: false,
        queue_mode: QueueMode::Notify,
        cdn_url: default_cdn_root_url(),
        feedback_url: default_feedback_url(),
        support_url: default_support_url(),
        wiki_url: default_wiki_url(),
    }
}

fn default_cdn_root_url() -> String {
    "http://127.0.0.1:8010".to_owned()
}

fn default_feedback_url() -> String {
    "https://mstdn.ca/@ngram".to_owned()
}

fn default_support_url() -> String {
    "https://rvlt.gg/jtVE0pD5".to_owned()
}

fn default_wiki_url() -> String {
    "https://git.ngram.ca/OpenJam/servers/wiki".to_owned()
}
