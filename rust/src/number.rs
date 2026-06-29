// This is free and unencumbered software released into the public domain.

#[cfg(feature = "complex")]
mod complex;
#[cfg(feature = "complex")]
pub use complex::*;

#[cfg(feature = "decimal")]
mod decimal;
#[cfg(feature = "decimal")]
pub use decimal::*;

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

#[cfg(feature = "rational")]
mod rational;
#[cfg(feature = "rational")]
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
