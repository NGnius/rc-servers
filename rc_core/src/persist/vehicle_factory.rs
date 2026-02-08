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
    #[serde(alias = "integrated")]
    BuiltIn,
    #[serde(alias = "online")]
    Web(WebFactorySettings),
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArcFactorySettings {
    pub uri: String,
    #[serde(default = "default_true")]
    pub show_expired: bool,
    #[serde(default)]
    pub override_cdn: bool,
    #[serde(default = "default_true")]
    pub spoof_username: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebFactorySettings {
    pub url: String,
}
