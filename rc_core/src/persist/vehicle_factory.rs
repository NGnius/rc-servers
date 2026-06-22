use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FactoryConfig {
    #[serde(default = "default_variant")]
    pub adapter: AdapterSettings,
    #[serde(default = "default_upload_limit")]
    pub upload_limit: i32,
}

impl super::config::SelfValidator for FactoryConfig {
    type Context = crate::ConfigImpl;
    fn validate(&self, _info: &mut super::config::ValidationInfo, _ctx: &Self::Context) -> bool {
        // TODO
        true
    }
}

impl super::config::RedactedClone for FactoryConfig {
    fn redacted_clone(&self) -> Self {
        Self {
            adapter: self.adapter.redacted_clone(),
            upload_limit: self.upload_limit,
        }
    }
}

fn default_variant() -> AdapterSettings {
    AdapterSettings::BuiltIn
}

fn default_upload_limit() -> i32 {
    4096
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

impl super::config::RedactedClone for AdapterSettings {
    fn redacted_clone(&self) -> Self {
        match self {
            Self::Arc(arc) => {
                Self::Arc(ArcFactorySettings {
                    uri: "[REDACTED]".to_owned(),
                    show_expired: arc.show_expired,
                    override_cdn: arc.override_cdn,
                    spoof_username: arc.spoof_username,
                })
            },
            x => x.clone(),
        }
    }
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
