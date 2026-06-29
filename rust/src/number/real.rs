// This is free and unencumbered software released into the public domain.

/// A real number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Real {
    #[cfg(feature = "decimal")]
    Decimal(super::Decimal),

    #[cfg(feature = "float")]
    Float(super::Float),

    #[cfg(feature = "integer")]
    Integer(super::Integer),

    #[cfg(feature = "integer")]
    Natural(super::Natural),

    #[cfg(feature = "rational")]
    Rational(super::Rational),
}

#[cfg(feature = "decimal")]
impl From<super::Decimal> for Real {
    fn from(input: super::Decimal) -> Self {
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
impl From<super::Float> for Real {
    fn from(input: super::Float) -> Self {
        Self::Float(input)
    }
}

#[cfg(feature = "integer")]
impl From<super::Integer> for Real {
    fn from(input: super::Integer) -> Self {
        Self::Integer(input)
    }
}

#[cfg(feature = "integer")]
impl From<super::Natural> for Real {
    fn from(input: super::Natural) -> Self {
        Self::Natural(input)
    }
}

#[cfg(feature = "rational")]
impl From<super::Rational> for Real {
    fn from(input: super::Rational) -> Self {
        Self::Rational(input)
    }
}

#[cfg(feature = "rational")]
impl From<(super::Integer, super::Integer)> for Real {
    fn from(input: (super::Integer, super::Integer)) -> Self {
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
