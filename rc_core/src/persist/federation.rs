use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FederationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_aliases")]
    pub aliases: std::collections::HashMap<String, String>,
    #[serde(default = "default_defederated")]
    pub defederated: Vec<String>,
}

pub fn default_federation() -> FederationConfig {
    FederationConfig {
        enabled: false,
        aliases: default_aliases(),
        defederated: default_defederated(),
    }
}

fn default_aliases() -> std::collections::HashMap<String, String> {
    let mut alias_map = std::collections::HashMap::with_capacity(3);
    alias_map.insert("rc.ngram.ca".to_owned(), "society.rc.ngram.ca".to_owned());
    alias_map.insert("robocraft.online".to_owned(), "society.robocraft.online".to_owned());
    alias_map.insert("robocraftgame.co.uk".to_owned(), "society.robocraftgame.co.uk".to_owned());
    alias_map.insert("127.0.0.1".to_owned(), "127.0.0.1:8002".to_owned());
    alias_map
}

fn default_defederated() -> Vec<String> {
    vec![
        "robocraftgame.com".to_owned(),
        #[cfg(debug_assertions)]
        { "127.0.0.1:8002".to_owned() },
        #[cfg(debug_assertions)]
        { "127.0.0.1".to_owned() },
    ]
}
