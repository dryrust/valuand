// This is free and unencumbered software released into the public domain.

/// A shorthand type alias for [`Natural`].
pub type Nat = Natural;

/// A natural number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub enum Natural {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl Natural {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::U8(i) => *i == 0,
            Self::U16(i) => *i == 0,
            Self::U32(i) => *i == 0,
            Self::U64(i) => *i == 0,
            Self::U128(i) => *i == 0,
        }
    }

    pub fn as_u128(&self) -> u128 {
        match self {
            Self::U8(i) => *i as _,
            Self::U16(i) => *i as _,
            Self::U32(i) => *i as _,
            Self::U64(i) => *i as _,
            Self::U128(i) => *i,
        }
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        self.clone().into_json().ok()
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> Result<serde_json::Value, Self> {
        use alloc::string::ToString;
        use serde_json::{Number, Value};
        Ok(serde_json::Value::Number(match self {
            Self::U8(z) => z.into(),
            Self::U16(z) => z.into(),
            Self::U32(z) => z.into(),
            Self::U64(z) => z.into(), // TODO: z > max
            Self::U128(z) => return Ok(serde_json::Value::String(z.to_string())),
        }))
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        self.clone().into_bson().ok()
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> Result<bson::Bson, Self> {
        use alloc::string::ToString;
        use bson::Bson;
        Ok(match self {
            Self::U8(n) => Bson::Int32(n.into()),
            Self::U16(n) => Bson::Int32(n.into()),
            Self::U32(n) => Bson::Int64(n.into()),
            Self::U64(n) => return Ok(Bson::String(n.to_string())), // TODO: Bson::Decimal128
            Self::U128(n) => return Ok(Bson::String(n.to_string())), // TODO: Bson::Decimal128
        })
    }
}

impl core::fmt::Display for Natural {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::U8(n) => n.fmt(f),
            Self::U16(n) => n.fmt(f),
            Self::U32(n) => n.fmt(f),
            Self::U64(n) => n.fmt(f),
            Self::U128(n) => n.fmt(f),
        }
    }
}

impl<T> From<&T> for Natural
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

include!("natural/u8.rs");
include!("natural/u16.rs");
include!("natural/u32.rs");
include!("natural/u64.rs");
include!("natural/u128.rs");
include!("natural/u256.rs");
include!("natural/usize.rs");
include!("natural/big.rs");

include!("natural/str.rs");

#[cfg(feature = "decimal")]
impl From<Natural> for rust_decimal::Decimal {
    fn from(input: Natural) -> Self {
        input.into()
    }
}
