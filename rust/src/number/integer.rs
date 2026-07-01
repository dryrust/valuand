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
    pub const fn is_zero(&self) -> bool {
        match self {
            Self::I8(z) => *z == 0,
            Self::I16(z) => *z == 0,
            Self::I32(z) => *z == 0,
            Self::I64(z) => *z == 0,
            Self::I128(z) => *z == 0,
        }
    }

    pub const fn is_one(&self) -> bool {
        match self {
            Self::I8(z) => *z == 1,
            Self::I16(z) => *z == 1,
            Self::I32(z) => *z == 1,
            Self::I64(z) => *z == 1,
            Self::I128(z) => *z == 1,
        }
    }

    pub const fn as_bool(&self) -> bool {
        !self.is_zero()
    }

    pub const fn as_f32(&self) -> f32 {
        match self {
            Self::I8(z) => *z as _,
            Self::I16(z) => *z as _,
            Self::I32(z) => *z as _,
            Self::I64(z) => *z as _,
            Self::I128(z) => *z as _,
        }
    }

    pub const fn as_f64(&self) -> f64 {
        match self {
            Self::I8(z) => *z as _,
            Self::I16(z) => *z as _,
            Self::I32(z) => *z as _,
            Self::I64(z) => *z as _,
            Self::I128(z) => *z as _,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_i8(&self) -> i8 {
        self.to_i8().unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn as_i16(&self) -> i16 {
        self.to_i16().unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn as_i32(&self) -> i32 {
        self.to_i32().unwrap()
    }

    #[allow(dead_code)]
    pub(crate) fn as_i64(&self) -> i64 {
        self.to_i64().unwrap()
    }

    pub(crate) const fn as_i128(&self) -> i128 {
        match self {
            Self::I8(z) => *z as _,
            Self::I16(z) => *z as _,
            Self::I32(z) => *z as _,
            Self::I64(z) => *z as _,
            Self::I128(z) => *z,
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

    pub const fn to_i8(&self) -> Option<i8> {
        match self {
            Self::I8(z) => Some(*z as _),
            Self::I16(z) if *z < i8::MIN as i16 || *z > i8::MAX as i16 => None,
            Self::I16(z) => Some(*z as _),
            Self::I32(z) if *z < i8::MIN as i32 || *z > i8::MAX as i32 => None,
            Self::I32(z) => Some(*z as _),
            Self::I64(z) if *z < i8::MIN as i64 || *z > i8::MAX as i64 => None,
            Self::I64(z) => Some(*z as _),
            Self::I128(z) if *z < i8::MIN as i128 || *z > i8::MAX as i128 => None,
            Self::I128(z) => Some(*z as _),
        }
    }

    pub const fn to_i16(&self) -> Option<i16> {
        match self {
            Self::I8(z) => Some(*z as _),
            Self::I16(z) => Some(*z as _),
            Self::I32(z) if *z < i16::MIN as i32 || *z > i16::MAX as i32 => None,
            Self::I32(z) => Some(*z as _),
            Self::I64(z) if *z < i16::MIN as i64 || *z > i16::MAX as i64 => None,
            Self::I64(z) => Some(*z as _),
            Self::I128(z) if *z < i16::MIN as i128 || *z > i16::MAX as i128 => None,
            Self::I128(z) => Some(*z as _),
        }
    }

    pub const fn to_i32(&self) -> Option<i32> {
        match self {
            Self::I8(z) => Some(*z as _),
            Self::I16(z) => Some(*z as _),
            Self::I32(z) => Some(*z as _),
            Self::I64(z) if *z < i32::MIN as i64 || *z > i32::MAX as i64 => None,
            Self::I64(z) => Some(*z as _),
            Self::I128(z) if *z < i32::MIN as i128 || *z > i32::MAX as i128 => None,
            Self::I128(z) => Some(*z as _),
        }
    }

    pub const fn to_i64(&self) -> Option<i64> {
        match self {
            Self::I8(z) => Some(*z as _),
            Self::I16(z) => Some(*z as _),
            Self::I32(z) => Some(*z as _),
            Self::I64(z) => Some(*z as _),
            Self::I128(z) if *z < i64::MIN as i128 || *z > i64::MAX as i128 => None,
            Self::I128(z) => Some(*z as _),
        }
    }

    pub const fn to_i128(&self) -> Option<i128> {
        Some(self.as_i128())
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

#[cfg(all(feature = "decimal", feature = "rust_decimal"))]
impl From<Integer> for rust_decimal::Decimal {
    fn from(input: Integer) -> Self {
        input.into()
    }
}
