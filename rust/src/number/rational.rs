// This is free and unencumbered software released into the public domain.

use super::Integer;

/// A rational number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rational {
    pub numerator: Integer,
    pub denominator: Integer,
}

impl From<Integer> for Rational {
    fn from(numerator: Integer) -> Self {
        Self {
            numerator,
            denominator: 1.into(),
        }
    }
}

impl From<(Integer, Integer)> for Rational {
    fn from((numerator, denominator): (Integer, Integer)) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl From<Rational> for (Integer, Integer) {
    fn from(input: Rational) -> Self {
        (input.numerator, input.denominator)
    }
}
