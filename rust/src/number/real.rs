// This is free and unencumbered software released into the public domain.

#[cfg(feature = "float")]
use decorum::Total;

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

#[cfg(feature = "float")]
impl From<f32> for Real {
    fn from(input: f32) -> Self {
        Self::Float(input.into())
    }
}

#[cfg(feature = "float")]
impl From<Total<f32>> for Real {
    fn from(input: Total<f32>) -> Self {
        Self::Float(input.into())
    }
}

#[cfg(feature = "float")]
impl From<f64> for Real {
    fn from(input: f64) -> Self {
        Self::Float(input.into())
    }
}

#[cfg(feature = "float")]
impl From<Total<f64>> for Real {
    fn from(input: Total<f64>) -> Self {
        Self::Float(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<Integer> for Real {
    fn from(input: Integer) -> Self {
        Self::Integer(input)
    }
}

#[cfg(feature = "integer")]
impl From<i8> for Real {
    fn from(input: i8) -> Self {
        Self::Integer(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<i16> for Real {
    fn from(input: i16) -> Self {
        Self::Integer(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<i32> for Real {
    fn from(input: i32) -> Self {
        Self::Integer(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<i64> for Real {
    fn from(input: i64) -> Self {
        Self::Integer(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<i128> for Real {
    fn from(input: i128) -> Self {
        Self::Integer(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<Natural> for Real {
    fn from(input: Natural) -> Self {
        Self::Natural(input)
    }
}

#[cfg(feature = "integer")]
impl From<u8> for Real {
    fn from(input: u8) -> Self {
        Self::Natural(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<u16> for Real {
    fn from(input: u16) -> Self {
        Self::Natural(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<u32> for Real {
    fn from(input: u32) -> Self {
        Self::Natural(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<u64> for Real {
    fn from(input: u64) -> Self {
        Self::Natural(input.into())
    }
}

#[cfg(feature = "integer")]
impl From<u128> for Real {
    fn from(input: u128) -> Self {
        Self::Natural(input.into())
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
