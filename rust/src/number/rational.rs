// This is free and unencumbered software released into the public domain.

use super::Integer;

/// A rational number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rational {
    pub numerator: Integer,
    pub denominator: Integer,
}
