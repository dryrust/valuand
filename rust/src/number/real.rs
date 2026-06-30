// This is free and unencumbered software released into the public domain.

/// A real number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl Real {
    #[cfg(feature = "decimal")]
    pub fn is_decimal(&self) -> bool {
        matches!(self, Real::Decimal(_))
    }

    #[cfg(feature = "float")]
    pub fn is_float(&self) -> bool {
        matches!(self, Real::Float(_))
    }

    #[cfg(feature = "integer")]
    pub fn is_integer(&self) -> bool {
        matches!(self, Real::Integer(_))
    }

    #[cfg(feature = "integer")]
    pub fn is_natural(&self) -> bool {
        matches!(self, Real::Natural(_))
    }

    #[cfg(feature = "rational")]
    pub fn is_rational(&self) -> bool {
        matches!(self, Real::Rational(_))
    }

    #[cfg(feature = "decimal")]
    pub fn to_decimal(&self) -> Option<super::Decimal> {
        match self {
            Self::Decimal(r) => Some(r.clone()),
            _ => None,
        }
    }

    #[cfg(feature = "float")]
    pub fn to_float(&self) -> Option<super::Float> {
        match self {
            Self::Float(r) => Some(r.clone()),
            _ => None,
        }
    }

    #[cfg(feature = "integer")]
    pub fn to_integer(&self) -> Option<super::Integer> {
        match self {
            Self::Integer(z) => Some(z.clone()),
            _ => None,
        }
    }

    #[cfg(feature = "integer")]
    pub fn to_natural(&self) -> Option<super::Natural> {
        match self {
            Self::Natural(n) => Some(n.clone()),
            _ => None,
        }
    }

    #[cfg(feature = "rational")]
    pub fn to_rational(&self) -> Option<super::Rational> {
        match self {
            Self::Rational(q) => Some(q.clone()),
            _ => None,
        }
    }

    #[cfg(feature = "decimal")]
    pub fn into_decimal(self) -> Result<super::Decimal, Self> {
        match self {
            Self::Decimal(r) => Ok(r),
            _ => Err(self),
        }
    }

    #[cfg(feature = "float")]
    pub fn into_float(self) -> Result<super::Float, Self> {
        match self {
            Self::Float(r) => Ok(r),
            _ => Err(self),
        }
    }

    #[cfg(feature = "integer")]
    pub fn into_integer(self) -> Result<super::Integer, Self> {
        match self {
            Self::Integer(z) => Ok(z),
            _ => Err(self),
        }
    }

    #[cfg(feature = "integer")]
    pub fn into_natural(self) -> Result<super::Natural, Self> {
        match self {
            Self::Natural(n) => Ok(n),
            _ => Err(self),
        }
    }

    #[cfg(feature = "rational")]
    pub fn into_rational(self) -> Result<super::Rational, Self> {
        match self {
            Self::Rational(q) => Ok(q),
            _ => Err(self),
        }
    }
}

impl<T> From<&T> for Real
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
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
include!("real/i256.rs");
include!("real/isize.rs");

include!("real/u8.rs");
include!("real/u16.rs");
include!("real/u32.rs");
include!("real/u64.rs");
include!("real/u128.rs");
include!("real/u256.rs");
include!("real/usize.rs");
