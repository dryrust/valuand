// This is free and unencumbered software released into the public domain.

/// A decimal number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal(
    #[cfg_attr(feature = "serde", serde(with = "rust_decimal::serde::str"))] rust_decimal::Decimal,
);

include!("decimal/i8.rs");
include!("decimal/i16.rs");
include!("decimal/i32.rs");
include!("decimal/i64.rs");
include!("decimal/i128.rs");

include!("decimal/u8.rs");
include!("decimal/u16.rs");
include!("decimal/u32.rs");
include!("decimal/u64.rs");
include!("decimal/u128.rs");

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
