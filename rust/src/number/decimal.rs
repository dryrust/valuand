// This is free and unencumbered software released into the public domain.

use rust_decimal::prelude::ToPrimitive;

/// A shorthand type alias for [`Decimal`].
pub type Dec = Decimal;

/// A decimal number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct Decimal(
    #[cfg_attr(feature = "serde", serde(with = "rust_decimal::serde::str"))] rust_decimal::Decimal,
);

impl Decimal {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_integer(&self) -> bool {
        return self.0.as_f64().fract() == 0.0;
    }

    pub fn as_f64(&self) -> f64 {
        return self.0.as_f64();
    }

    pub fn to_f64(&self) -> Option<f64> {
        Some(self.as_f64())
    }

    pub fn to_i128(&self) -> Option<i128> {
        if !self.is_integer() {
            return None;
        }
        return self.0.to_i128();
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        self.clone().into_json().ok()
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> Result<serde_json::Value, Self> {
        use alloc::string::ToString;
        Ok(serde_json::Value::String(self.to_string()))
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        self.clone().into_bson().ok()
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> Result<bson::Bson, Self> {
        use alloc::string::ToString;
        Ok(bson::Bson::String(self.to_string())) // TODO: Bson::Decimal128
    }
}

impl core::fmt::Display for Decimal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<&T> for Decimal
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

include!("decimal/f32.rs");
include!("decimal/f64.rs");

include!("decimal/i8.rs");
include!("decimal/i16.rs");
include!("decimal/i32.rs");
include!("decimal/i64.rs");
include!("decimal/i128.rs");
include!("decimal/i256.rs");
include!("decimal/isize.rs");

include!("decimal/u8.rs");
include!("decimal/u16.rs");
include!("decimal/u32.rs");
include!("decimal/u64.rs");
include!("decimal/u128.rs");
include!("decimal/u256.rs");
include!("decimal/usize.rs");

include!("decimal/str.rs");

impl From<rust_decimal::Decimal> for Decimal {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self(input)
    }
}

impl From<Decimal> for rust_decimal::Decimal {
    fn from(input: Decimal) -> Self {
        input.0
    }
}
