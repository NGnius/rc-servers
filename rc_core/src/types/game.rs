use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TeamId(i32);

impl super::RawType for TeamId {
    type Raw = i32;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TeamId({})", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerId(u8);

impl super::RawType for PlayerId {
    type Raw = u8;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerId({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameGuid(i64);

impl GameGuid {
    pub fn from_guid_str(guid_str: &str) -> Option<Self> {
        crate::persist::user::str_to_i64(guid_str).map(Self)
    }

    pub fn as_guid_str(&self) -> String {
        crate::persist::user::i64_as_uuid_str(self.0)
    }
}

impl super::RawType for GameGuid {
    type Raw = i64;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for GameGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameGuid({}|{})", self.0, self.as_guid_str())
    }
}
