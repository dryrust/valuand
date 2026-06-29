// This is free and unencumbered software released into the public domain.

/// A floating-point number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Float {
    //F16(F16),
    F32(F32),
    F64(F64),
}

mod f16;
pub use f16::*;

mod f32;
pub use f32::*;

mod f64;
pub use f64::*;
