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

impl super::config::SelfValidator for Settings {
    type Context = crate::ConfigImpl;
    fn validate(&self, _info: &mut super::config::ValidationInfo, _ctx: &Self::Context) -> bool {
        // TODO
        true
    }
}

impl super::config::RedactedClone for Settings {
    fn redacted_clone(&self) -> Self {
        Self {
            gameplay: self.gameplay.clone(),
            banners: self.banners.clone(),
            garage_upgrades: self.garage_upgrades.clone(),
            server: self.server.redacted_clone(),
        }
    }
}

fn default_gameplay_settings() -> super::GameplaySettings {
    super::GameplaySettings {
        show_tutorial_after_date: "2077-01-01".to_owned(),
        health_threshold: 0.20,
        microbot_sphere: 0.5,
        misfire_angle: 10.0,
        shield_dps: 100_000,
        shield_hps: 10_000_000,
        request_review_level: 10_000,
        critical_ratio: 5.0,
        cross_promo_image: "https://git.ngram.ca/OpenJam/rc-servers/raw/branch/main/assets/robocraft/default.png".to_owned(),
        cross_promo_link: "https://git.ngram.ca/OpenJam/rc-servers".to_owned(),
        garages_limit: 100,
        platform: super::client_config::default_platform_conf(),
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
            cost: 0, // the first needs to be zero cost otherwise the client doesn't do upgrade cost math correctly
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
    #[serde(default = "default_true")]
    pub allow_signup: bool,
    #[serde(default)]
    pub queue_mode: QueueMode,
    #[serde(default = "default_domain_root")]
    pub domain: String,
    #[serde(default = "default_cdn_root_url")]
    pub cdn_url: String,
    #[serde(default = "default_auth_root_url")]
    pub auth_url: String,
    #[serde(default = "default_intercom_root_url")]
    pub intercom_url: String,
    #[serde(default = "default_factory_url")]
    pub factory_url: String,
    #[serde(default = "default_feedback_url")]
    pub feedback_url: String,
    #[serde(default = "default_support_url")]
    pub support_url: String,
    #[serde(default = "default_wiki_url")]
    pub wiki_url: String,
    #[serde(default = "default_game_version")]
    pub min_version: u32,
    #[serde(default = "default_true")]
    pub dos_protection: bool,
    #[serde(default)]
    pub maintenance_message: Option<String>,
}

impl super::config::RedactedClone for ServerSettings {
    fn redacted_clone(&self) -> Self {
        let mut redacted = self.clone();
        redacted.database = "[REDACTED]".to_owned();
        redacted.intercom_url = "[REDACTED]".to_owned();
        redacted.dos_protection = true;
        redacted
    }
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
        allow_signup: true,
        queue_mode: QueueMode::Notify,
        domain: default_domain_root(),
        cdn_url: default_cdn_root_url(),
        auth_url: default_auth_root_url(),
        intercom_url: default_intercom_root_url(),
        factory_url: default_factory_url(),
        feedback_url: default_feedback_url(),
        support_url: default_support_url(),
        wiki_url: default_wiki_url(),
        min_version: default_game_version(),
        dos_protection: default_true(),
        maintenance_message: None,
    }
}

fn default_domain_root() -> String {
    "127.0.0.1".to_owned()
}

fn default_cdn_root_url() -> String {
    "http://127.0.0.1:8010".to_owned()
}

fn default_auth_root_url() -> String {
    "http://127.0.0.1:8001".to_owned() // mostly used for intercom
}

fn default_intercom_root_url() -> String {
    "ws://127.0.0.1:8001".to_owned()
}

fn default_factory_url() -> String {
    "http://127.0.0.1:8012".to_owned()
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

fn default_game_version() -> u32 {
    2855
}

fn default_true() -> bool {
    true
}
