// This is free and unencumbered software released into the public domain.

use super::Integer;

/// A shorthand type alias for [`Rational`].
pub type Rat = Rational;

/// A rational number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rational {
    pub numerator: Integer,
    pub denominator: Integer,
}

impl Rational {
    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
}

impl<T> From<&T> for Rational
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<Integer> for Rational {
    fn from(input: Integer) -> Self {
        Self {
            numerator: input,
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

include!("rational/i8.rs");
include!("rational/i16.rs");
include!("rational/i32.rs");
include!("rational/i64.rs");
include!("rational/i128.rs");
include!("rational/i256.rs");
include!("rational/isize.rs");
