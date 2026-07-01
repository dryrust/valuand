// This is free and unencumbered software released into the public domain.

use super::{F32, F64};
use decorum::Total;
use num_traits::identities::Zero;

/// A floating-point number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Float {
    // TODO: F16(F16),
    F32(F32),
    F64(F64),
}

impl Float {
    pub const fn is_zero(&self) -> bool {
        match self {
            Float::F32(f) => f.is_zero(),
            Float::F64(f) => f.is_zero(),
        }
    }

    pub const fn is_one(&self) -> bool {
        match self {
            Float::F32(f) => f.is_one(),
            Float::F64(f) => f.is_one(),
        }
    }

    pub const fn as_bool(&self) -> bool {
        !self.is_zero()
    }

    pub const fn as_f32(&self) -> f32 {
        match self {
            Float::F32(f) => f.into_inner(),
            Float::F64(f) => f.into_inner() as _,
        }
    }

    pub const fn as_f64(&self) -> f64 {
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

impl From<Float> for f64 {
    fn from(input: Float) -> Self {
        input.as_f64()
    }
}

include!("float/f32.rs");
include!("float/f64.rs");

include!("float/borsh.rs");
