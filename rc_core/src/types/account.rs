use serde::{Serialize, Deserialize};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct PublicId(String);

impl super::RawType for PublicId {
    type Raw = String;

    fn as_raw(&self) -> Self::Raw {
        self.0.clone()
    }

    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for PublicId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PublicId({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct DisplayName(String);

impl super::RawType for DisplayName {
    type Raw = String;

    fn as_raw(&self) -> Self::Raw {
        self.0.clone()
    }

    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DisplayName({})", self.0)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UserId(i32);

impl super::RawType for UserId {
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

impl core::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserId({})", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct Currency<T>(T);

impl super::RawType for Currency<u64> {
    type Raw = u64;

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

impl super::RawType for Currency<i64> {
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
