use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SanctionJson {
    #[serde(rename="Type")]
    pub type_: SanctionType,
    #[serde(rename="Reason")]
    pub reason: String,
    #[serde(rename="Reporter")]
    pub reporter: String,
    #[serde(rename="Issued", serialize_with="serde_issued::serialize")]
    pub issued: chrono::DateTime<chrono::Utc>,
}

impl SanctionJson {
    pub fn as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

mod serde_issued {
    use chrono::{DateTime, Utc};

    /*pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
        d.deserialize_str(todo!())
    }*/

    pub fn serialize<S: serde::Serializer>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&dt.to_rfc2822())
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum SanctionType {
    Warning = 0,
    Mute = 1,
    Suspension = 2,
    Note = 3,
    Kick = 4,
}

impl SanctionType {
    pub(crate) fn from_db(t: rc_database::schema::sanction::Descriptor) -> Self {
        match t {
            rc_database::schema::sanction::Descriptor::Warn => Self::Warning,
            rc_database::schema::sanction::Descriptor::Mute => Self::Mute,
            rc_database::schema::sanction::Descriptor::Ban => Self::Suspension,
            rc_database::schema::sanction::Descriptor::Note => Self::Note,
            rc_database::schema::sanction::Descriptor::Kick => Self::Kick,
        }
    }

    pub(crate) fn to_db(self) -> rc_database::schema::sanction::Descriptor {
        match self {
            Self::Warning => rc_database::schema::sanction::Descriptor::Warn,
            Self::Mute => rc_database::schema::sanction::Descriptor::Mute,
            Self::Suspension => rc_database::schema::sanction::Descriptor::Ban,
            Self::Note => rc_database::schema::sanction::Descriptor::Note,
            Self::Kick => rc_database::schema::sanction::Descriptor::Kick,
        }
    }

    pub(crate) fn from_persist(t: crate::persist::user::SanctionType) -> Self {
        match t {
            crate::persist::user::SanctionType::Warn => Self::Warning,
            crate::persist::user::SanctionType::Mute => Self::Mute,
            crate::persist::user::SanctionType::Ban => Self::Suspension,
            crate::persist::user::SanctionType::Note => Self::Note,
            crate::persist::user::SanctionType::Kick => Self::Kick,
        }
    }
}
