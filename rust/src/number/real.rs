// This is free and unencumbered software released into the public domain.

use super::{Decimal, Float, Integer, Natural, Rational};

/// A real number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Real {
    #[cfg(feature = "decimal")]
    Decimal(Decimal),

    #[cfg(feature = "float")]
    Float(Float),

    #[cfg(feature = "integer")]
    Integer(Integer),

    #[cfg(feature = "integer")]
    Natural(Natural),

    #[cfg(feature = "rational")]
    Rational(Rational),
}

#[cfg(feature = "decimal")]
impl From<Decimal> for Real {
    fn from(input: Decimal) -> Self {
        Self::Decimal(input)
    }
}

#[cfg(feature = "decimal")]
impl From<rust_decimal::Decimal> for Real {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

#[cfg(feature = "float")]
impl From<Float> for Real {
    fn from(input: Float) -> Self {
        Self::Float(input)
    }
}

#[cfg(feature = "integer")]
impl From<Integer> for Real {
    fn from(input: Integer) -> Self {
        Self::Integer(input)
    }
}

#[cfg(feature = "integer")]
impl From<Natural> for Real {
    fn from(input: Natural) -> Self {
        Self::Natural(input)
    }
}

#[cfg(feature = "rational")]
impl From<Rational> for Real {
    fn from(input: Rational) -> Self {
        Self::Rational(input)
    }
}

#[cfg(feature = "rational")]
impl From<(Integer, Integer)> for Real {
    fn from(input: (Integer, Integer)) -> Self {
        Self::Rational(input.into())
    }
}

include!("real/f32.rs");
include!("real/f64.rs");

include!("real/i8.rs");
include!("real/i16.rs");
include!("real/i32.rs");
include!("real/i64.rs");
include!("real/i128.rs");

include!("real/u8.rs");
include!("real/u16.rs");
include!("real/u32.rs");
include!("real/u64.rs");
include!("real/u128.rs");
