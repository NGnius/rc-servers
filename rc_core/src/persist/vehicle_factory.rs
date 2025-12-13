use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FactoryConfig {
    #[serde(default = "default_variant")]
    pub adapter: AdapterSettings,
}

impl super::config::SelfValidator for FactoryConfig {
    type Context = crate::ConfigImpl;
    fn validate(&self, _info: &mut super::config::ValidationInfo, _ctx: &Self::Context) -> bool {
        // TODO
        true
    }
}

fn default_variant() -> AdapterSettings {
    AdapterSettings::None
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "variant")]
pub enum AdapterSettings {
    #[serde(alias = "sqlite")]
    Arc(ArcFactorySettings),
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArcFactorySettings {
    pub uri: String,
    #[serde(default = "default_true")]
    pub show_expired: bool,
    /// should probably end with /roboshop/arc/Live/
    #[serde(default = "default_arc_live_url")]
    pub cdn: String,
    #[serde(default)]
    pub override_cdn: bool,
    #[serde(default = "default_true")]
    pub spoof_username: bool,
}

fn default_arc_live_url() -> String {
    format!("{}/roboshop/arc/Live/", super::settings::default_cdn_root_url())
}

fn default_true() -> bool {
    true
}
