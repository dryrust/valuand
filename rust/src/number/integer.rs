// This is free and unencumbered software released into the public domain.

/// A shorthand type alias for [`Integer`].
pub type Int = Integer;

/// An integer number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub enum Integer {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

impl Integer {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::I8(z) => *z == 0,
            Self::I16(z) => *z == 0,
            Self::I32(z) => *z == 0,
            Self::I64(z) => *z == 0,
            Self::I128(z) => *z == 0,
        }
    }

    pub fn as_i128(&self) -> i128 {
        match self {
            Self::I8(z) => *z as _,
            Self::I16(z) => *z as _,
            Self::I32(z) => *z as _,
            Self::I64(z) => *z as _,
            Self::I128(z) => *z,
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
            Self::I8(z) => z.into(),
            Self::I16(z) => z.into(),
            Self::I32(z) => z.into(),
            Self::I64(z) => z.into(), // TODO: z > max
            Self::I128(z) => return Ok(serde_json::Value::String(z.to_string())),
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
            Self::I8(z) => Bson::Int32(z.into()),
            Self::I16(z) => Bson::Int32(z.into()),
            Self::I32(z) => Bson::Int32(z.into()),
            Self::I64(z) => Bson::Int64(z.into()),
            Self::I128(z) => return Ok(Bson::String(z.to_string())), // TODO: Bson::Decimal128
        })
    }
}

impl core::fmt::Display for Integer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::I8(z) => z.fmt(f),
            Self::I16(z) => z.fmt(f),
            Self::I32(z) => z.fmt(f),
            Self::I64(z) => z.fmt(f),
            Self::I128(z) => z.fmt(f),
        }
    }
}

impl<T> From<&T> for Integer
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

include!("integer/i8.rs");
include!("integer/i16.rs");
include!("integer/i32.rs");
include!("integer/i64.rs");
include!("integer/i128.rs");
include!("integer/i256.rs");
include!("integer/isize.rs");
include!("integer/big.rs");

include!("integer/str.rs");

#[cfg(feature = "decimal")]
impl From<Integer> for rust_decimal::Decimal {
    fn from(input: Integer) -> Self {
        input.into()
    }
}
