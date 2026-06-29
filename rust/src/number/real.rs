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
