use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemType {
    OrderedCollection,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox<T> {
    #[serde(rename = "type")]
    pub kind: ItemType,
    pub id: String,
    pub total_items: u32,
    pub ordered_items: Vec<T>,
    #[serde(default)]
    pub endpoints: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<oj_serdes::society::activitypub::Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<oj_serdes::society::activitypub::Image>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActivityType {
    Create,
    Update,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity<T> {
    #[serde(rename = "type")]
    pub kind: ActivityType,
    pub id: String,
    pub actor: String,
    pub object: T,
    pub published: chrono::DateTime<chrono::Utc>,
    pub to: String,
    pub cc: Vec<String>,
}
