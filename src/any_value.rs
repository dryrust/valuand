// This is free and unencumbered software released into the public domain.

use super::{Value, ValueType};

/// A sum type that can hold values of any type, including `Any`.
pub type AnyValue = Value<alloc::boxed::Box<dyn core::any::Any>>;

/// A type discriminator for an [AnyValue].
pub type AnyValueType = ValueType;
