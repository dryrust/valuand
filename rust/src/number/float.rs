// This is free and unencumbered software released into the public domain.

use decorum::Total;
use num_traits::identities::Zero;

/// A floating-point number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub enum Float {
    // TODO: F16(F16),
    F32(F32),
    F64(F64),
}

impl Float {
    pub fn is_zero(&self) -> bool {
        match self {
            Float::F32(f) => f.is_zero(),
            Float::F64(f) => f.is_zero(),
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            Float::F32(f) => f.into_inner(),
            Float::F64(f) => f.into_inner() as _,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Float::F32(f) => f.into_inner() as _,
            Float::F64(f) => f.into_inner(),
        }
    }
}

impl<T> From<&T> for Float
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<f32> for Float {
    fn from(input: f32) -> Self {
        Self::F32(input.into())
    }
}

impl From<Total<f32>> for Float {
    fn from(input: Total<f32>) -> Self {
        Self::F32(input)
    }
}

impl From<f64> for Float {
    fn from(input: f64) -> Self {
        Self::F64(input.into())
    }
}

impl From<Total<f64>> for Float {
    fn from(input: Total<f64>) -> Self {
        Self::F64(input)
    }
}

impl From<Float> for f64 {
    fn from(input: Float) -> Self {
        input.as_f64()
    }
}

mod f16;
pub use f16::*;

mod f32;
pub use f32::*;

mod f64;
pub use f64::*;
