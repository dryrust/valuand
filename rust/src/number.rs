// This is free and unencumbered software released into the public domain.

#[cfg(feature = "complex")]
mod complex;
#[cfg(feature = "complex")]
pub use complex::*;

#[cfg(feature = "decimal")]
mod decimal;
#[cfg(feature = "decimal")]
pub use decimal::*;

#[cfg(feature = "decimal")]
mod decimal_error;
#[cfg(feature = "decimal")]
pub use decimal_error::*;

#[cfg(feature = "float")]
mod f16;
#[cfg(feature = "float")]
pub use f16::*;

#[cfg(feature = "float")]
mod f32;
#[cfg(feature = "float")]
pub use f32::*;

#[cfg(feature = "float")]
mod f64;
#[cfg(feature = "float")]
pub use f64::*;

#[cfg(feature = "float")]
mod float;
#[cfg(feature = "float")]
pub use float::*;

#[cfg(feature = "integer")]
mod integer;
#[cfg(feature = "integer")]
pub use integer::*;

#[cfg(feature = "integer")]
mod natural;
#[cfg(feature = "integer")]
pub use natural::*;

#[cfg(all(feature = "rational", feature = "integer"))]
mod rational;
#[cfg(all(feature = "rational", feature = "integer"))]
pub use rational::*;

#[cfg(any(
    feature = "decimal",
    feature = "float",
    feature = "integer",
    feature = "rational"
))]
mod real;
#[cfg(any(
    feature = "decimal",
    feature = "float",
    feature = "integer",
    feature = "rational"
))]
pub use real::*;
