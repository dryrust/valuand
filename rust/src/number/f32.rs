// This is free and unencumbered software released into the public domain.

use core::{num::ParseFloatError, str::FromStr};
use decorum::Total;

/// A totally-ordered 32-bit floating-point number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct F32(Total<f32>);

impl F32 {
    pub const fn is_zero(&self) -> bool {
        self.0.into_inner() == 0.0
    }

    pub const fn is_one(&self) -> bool {
        self.0.into_inner() == 1.0
    }

    pub const fn as_bool(&self) -> bool {
        !self.is_zero()
    }

    pub const fn as_f32(&self) -> f32 {
        self.0.into_inner()
    }

    pub const fn as_f64(&self) -> f64 {
        self.0.into_inner() as _
    }

    pub const fn into_inner(self) -> f32 {
        self.0.into_inner()
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Number::from_f64(self.as_f64())
            .map(serde_json::Value::Number)
            .unwrap()
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::Double(self.as_f64())
    }
}

impl core::fmt::Display for F32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let v = self.as_f32();
        if v.is_nan() {
            // XML Schema lexical form for NaN
            return write!(f, "NaN");
        }
        if v.is_infinite() {
            // XML Schema lexical forms for infinities: INF and -INF
            return if v.is_sign_positive() {
                write!(f, "INF")
            } else {
                write!(f, "-INF")
            };
        }
        // Finite values: delegate to f64's formatting (preserves sign of zero)
        write!(f, "{}", v)
    }
}

impl<T> From<&T> for F32
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl FromStr for F32 {
    type Err = ParseFloatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<Total<f32>>().map(Self)
    }
}

include!("f32/f32.rs");
include!("f32/f64.rs");

include!("f32/isize.rs");
include!("f32/usize.rs");

include!("f32/borsh.rs");
include!("f32/bson.rs");
