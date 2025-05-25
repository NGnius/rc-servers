use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FactoryConfig {
    #[serde(default = "default_variant")]
    pub adapter: AdapterSettings,
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
    #[serde(default)]
    pub override_cdn: Option<String>,
}

fn default_true() -> bool {
    true
}
