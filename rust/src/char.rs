// This is free and unencumbered software released into the public domain.

/// A Unicode scalar value (aka code point).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Char(pub char);

impl Char {
    pub const fn as_bool(&self) -> bool {
        self.0 == '\0'
    }

    pub const fn as_i32(&self) -> i32 {
        self.0 as _
    }

    pub const fn as_i64(&self) -> i64 {
        self.0 as _
    }

    pub const fn as_i128(&self) -> i128 {
        self.0 as _
    }

    pub const fn as_u32(&self) -> u32 {
        self.0 as _
    }

    pub const fn as_u64(&self) -> u64 {
        self.0 as _
    }

    pub const fn as_u128(&self) -> u128 {
        self.0 as _
    }

    pub const fn into_inner(self) -> char {
        self.0
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Value::Number(self.as_i32().into())
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::Int32(self.as_i32())
    }
}

impl core::fmt::Display for Char {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<&T> for Char
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

include!("char/char.rs");
include!("char/u32.rs");

include!("char/borsh.rs");
include!("char/bson.rs");
