// This is free and unencumbered software released into the public domain.

use super::{Scalar, ScalarType};

/// A sum type that can hold values of any type, including `Any`.
pub type AnyScalar = Scalar<alloc::boxed::Box<dyn core::any::Any>>;

/// A type discriminator for an [`AnyScalar`].
pub type AnyScalarType = ScalarType;
