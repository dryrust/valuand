// This is free and unencumbered software released into the public domain.

use super::ScalarType;
use core::{any::TypeId, fmt::Debug};

#[doc(hidden)]
#[derive(Debug)]
pub struct T;

/// A sum type that can hold values of any static type.
#[derive(Debug, Default)]
pub enum Scalar<T: Debug = self::T> {
    #[default]
    Unit,

    Bool(bool),

    #[cfg(feature = "number")]
    Number(super::Real),

    Char(char),

    Other(T),
}

impl<T> Scalar<T>
where
    T: Debug + 'static,
{
    pub fn r#type(&self) -> ScalarType {
        self.into()
    }

    pub fn type_id(&self) -> TypeId {
        self.r#type().type_id()
    }
}

impl<T> Scalar<T>
where
    T: Debug,
{
    pub const fn unit() -> Self {
        Self::Unit
    }

    pub fn bool(value: impl Into<bool>) -> Self {
        Self::Bool(value.into())
    }

    #[cfg(feature = "number")]
    pub fn number(value: impl Into<super::Real>) -> Self {
        Self::Number(value.into())
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Scalar::Unit)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Scalar::Bool(_))
    }

    #[cfg(feature = "number")]
    pub fn is_number(&self) -> bool {
        matches!(self, Scalar::Number(_))
    }

    #[cfg(all(feature = "number", feature = "decimal"))]
    pub fn is_decimal(&self) -> bool {
        use super::Real;
        matches!(self, Scalar::Number(Real::Decimal(_)))
    }

    #[cfg(all(feature = "number", feature = "float"))]
    pub fn is_float(&self) -> bool {
        use super::Real;
        matches!(self, Scalar::Number(Real::Float(_)))
    }

    #[cfg(all(feature = "number", feature = "integer"))]
    pub fn is_integer(&self) -> bool {
        use super::Real;
        matches!(self, Scalar::Number(Real::Integer(_)))
    }

    #[cfg(all(feature = "number", feature = "integer"))]
    pub fn is_natural(&self) -> bool {
        use super::Real;
        matches!(self, Scalar::Number(Real::Natural(_)))
    }

    #[cfg(all(feature = "number", feature = "rational"))]
    pub fn is_rational(&self) -> bool {
        use super::Real;
        matches!(self, Scalar::Number(Real::Rational(_)))
    }

    pub fn is_other(&self) -> bool {
        matches!(self, Scalar::Other(_))
    }

    pub fn as_unit(&self) -> Option<()> {
        self.to_unit()
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Scalar::Bool(value) => Some(value),
            _ => None,
        }
    }

    #[cfg(feature = "number")]
    pub fn as_number(&self) -> Option<&super::Real> {
        match self {
            Self::Number(number) => Some(number),
            _ => None,
        }
    }

    pub fn to_unit(&self) -> Option<()> {
        match self {
            Self::Unit => Some(()),
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    #[cfg(feature = "number")]
    pub fn to_number(&self) -> Option<super::Real> {
        match self {
            Self::Number(number) => Some(number.clone()),
            _ => None,
        }
    }

    pub fn into_unit(self) -> Result<(), Self> {
        match self {
            Self::Unit => Ok(()),
            _ => Err(self),
        }
    }

    pub fn into_bool(self) -> Result<bool, Self> {
        match self {
            Self::Bool(value) => Ok(value),
            _ => Err(self),
        }
    }

    #[cfg(feature = "number")]
    pub fn into_number(self) -> Result<super::Real, Self> {
        match self {
            Self::Number(number) => Ok(number),
            _ => Err(self),
        }
    }

    pub fn unwrap_unit(self) -> () {
        self.into_unit()
            .expect("unwrap_unit() should be called on a Scalar::Unit value")
    }

    pub fn unwrap_bool(self) -> bool {
        self.into_bool()
            .expect("unwrap_bool() should be called on a Scalar::Bool value")
    }

    #[cfg(feature = "number")]
    pub fn unwrap_number(self) -> super::Real {
        self.into_number()
            .expect("unwrap_number() should be called on a Scalar::Number value")
    }
}

impl<T, U> From<&U> for Scalar<T>
where
    T: Debug,
    U: Clone + Into<Self>,
{
    fn from(t: &U) -> Self {
        t.clone().into()
    }
}

include!("scalar/unit.rs");
include!("scalar/bool.rs");

include!("scalar/f32.rs");
include!("scalar/f64.rs");

include!("scalar/i8.rs");
include!("scalar/i16.rs");
include!("scalar/i32.rs");
include!("scalar/i64.rs");
include!("scalar/i128.rs");

include!("scalar/u8.rs");
include!("scalar/u16.rs");
include!("scalar/u32.rs");
include!("scalar/u64.rs");
include!("scalar/u128.rs");
