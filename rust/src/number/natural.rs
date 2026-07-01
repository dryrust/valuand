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
    pub const fn is_zero(&self) -> bool {
        match self {
            Self::U8(n) => *n == 0,
            Self::U16(n) => *n == 0,
            Self::U32(n) => *n == 0,
            Self::U64(n) => *n == 0,
            Self::U128(n) => *n == 0,
        }
    }

    pub const fn is_one(&self) -> bool {
        match self {
            Self::U8(n) => *n == 1,
            Self::U16(n) => *n == 1,
            Self::U32(n) => *n == 1,
            Self::U64(n) => *n == 1,
            Self::U128(n) => *n == 1,
        }
    }

    pub const fn as_bool(&self) -> bool {
        !self.is_zero()
    }

    pub const fn as_f32(&self) -> f32 {
        match self {
            Self::U8(n) => *n as _,
            Self::U16(n) => *n as _,
            Self::U32(n) => *n as _,
            Self::U64(n) => *n as _,
            Self::U128(n) => *n as _,
        }
    }

    pub const fn as_f64(&self) -> f64 {
        match self {
            Self::U8(n) => *n as _,
            Self::U16(n) => *n as _,
            Self::U32(n) => *n as _,
            Self::U64(n) => *n as _,
            Self::U128(n) => *n as _,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_u8(&self) -> u8 {
        self.to_u8().unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn as_u16(&self) -> u16 {
        self.to_u16().unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn as_u32(&self) -> u32 {
        self.to_u32().unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn as_u64(&self) -> u64 {
        self.to_u64().unwrap()
    }

    pub(crate) const fn as_u128(&self) -> u128 {
        match self {
            Self::U8(n) => *n as _,
            Self::U16(n) => *n as _,
            Self::U32(n) => *n as _,
            Self::U64(n) => *n as _,
            Self::U128(n) => *n,
        }
    }

    pub const fn to_bool(&self) -> Option<bool> {
        Some(self.as_bool())
    }

    pub const fn to_f32(&self) -> Option<f32> {
        Some(self.as_f32())
    }

    pub const fn to_f64(&self) -> Option<f64> {
        Some(self.as_f64())
    }

    pub const fn to_u8(&self) -> Option<u8> {
        match self {
            Self::U8(n) => Some(*n as _),
            Self::U16(n) if *n > u8::MAX as u16 => None,
            Self::U16(n) => Some(*n as _),
            Self::U32(n) if *n > u8::MAX as u32 => None,
            Self::U32(n) => Some(*n as _),
            Self::U64(n) if *n > u8::MAX as u64 => None,
            Self::U64(n) => Some(*n as _),
            Self::U128(n) if *n > u8::MAX as u128 => None,
            Self::U128(n) => Some(*n as _),
        }
    }

    pub const fn to_u16(&self) -> Option<u16> {
        match self {
            Self::U8(n) => Some(*n as _),
            Self::U16(n) => Some(*n as _),
            Self::U32(n) if *n > u16::MAX as u32 => None,
            Self::U32(n) => Some(*n as _),
            Self::U64(n) if *n > u16::MAX as u64 => None,
            Self::U64(n) => Some(*n as _),
            Self::U128(n) if *n > u16::MAX as u128 => None,
            Self::U128(n) => Some(*n as _),
        }
    }

    pub const fn to_u32(&self) -> Option<u32> {
        match self {
            Self::U8(n) => Some(*n as _),
            Self::U16(n) => Some(*n as _),
            Self::U32(n) => Some(*n as _),
            Self::U64(n) if *n > u32::MAX as u64 => None,
            Self::U64(n) => Some(*n as _),
            Self::U128(n) if *n > u32::MAX as u128 => None,
            Self::U128(n) => Some(*n as _),
        }
    }

    pub const fn to_u64(&self) -> Option<u64> {
        match self {
            Self::U8(n) => Some(*n as _),
            Self::U16(n) => Some(*n as _),
            Self::U32(n) => Some(*n as _),
            Self::U64(n) => Some(*n as _),
            Self::U128(n) if *n > u64::MAX as u128 => None,
            Self::U128(n) => Some(*n as _),
        }
    }

    pub const fn to_u128(&self) -> Option<u128> {
        Some(self.as_u128())
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
            Self::U8(n) => n.into(),
            Self::U16(n) => n.into(),
            Self::U32(n) => n.into(),
            Self::U64(n) => n.into(), // TODO: z > max
            Self::U128(n) => return Ok(serde_json::Value::String(n.to_string())),
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

#[cfg(all(feature = "decimal", feature = "rust_decimal"))]
impl From<Natural> for rust_decimal::Decimal {
    fn from(input: Natural) -> Self {
        input.into()
    }
}
