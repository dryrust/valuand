// This is free and unencumbered software released into the public domain.

//! This crate implements a universal scalar value (aka variant) type for Rust.

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod any_value;
#[cfg(feature = "alloc")]
pub use any_value::*;

mod value;
pub use value::*;

mod value_type;
pub use value_type::*;

#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
