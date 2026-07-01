// This is free and unencumbered software released into the public domain.

use super::{Integer, Natural};
use num_traits::One;
use rust_decimal::prelude::ToPrimitive;

/// A shorthand type alias for [`Decimal`].
pub type Dec = Decimal;

/// A decimal number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal(
    #[cfg_attr(feature = "serde", serde(with = "rust_decimal::serde::str"))] rust_decimal::Decimal,
);

impl Decimal {
    pub const ZERO: Self = Self(rust_decimal::Decimal::ZERO);
    pub const ONE: Self = Self(rust_decimal::Decimal::ONE);
    pub const TWO: Self = Self(rust_decimal::Decimal::TWO);
    pub const TEN: Self = Self(rust_decimal::Decimal::TEN);

    #[cfg(feature = "math")]
    pub const PI: Self = Self(rust_decimal::Decimal::PI);
    #[cfg(feature = "math")]
    pub const HALF_PI: Self = Self(rust_decimal::Decimal::HALF_PI);

    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn is_integer(&self) -> bool {
        self.0.is_integer()
    }

    pub fn as_bool(&self) -> bool {
        !self.is_zero()
    }

    pub fn as_f32(&self) -> f32 {
        self.0.as_f64() as _
    }

    pub fn as_f64(&self) -> f64 {
        self.0.as_f64()
    }

    pub fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }

    pub fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    pub fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    pub fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    pub fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    pub fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    pub fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    pub fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    pub fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    pub fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    pub fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    pub fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
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

impl From<Integer> for Decimal {
    fn from(input: Integer) -> Self {
        Self(input.as_i128().into())
    }
}

impl From<Natural> for Decimal {
    fn from(input: Natural) -> Self {
        Self(input.as_u128().into())
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

include!("decimal/borsh.rs");
include!("decimal/bson.rs");
include!("decimal/rust_decimal.rs");
