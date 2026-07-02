// This is free and unencumbered software released into the public domain.

//! This crate implements a universal scalar value (aka variant) type for Rust.

#![no_std]
#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod any_scalar;
#[cfg(feature = "alloc")]
pub use any_scalar::*;

mod bool;
pub use bool::*;

mod char;
pub use char::*;

#[cfg(feature = "number")]
mod number;
#[cfg(feature = "number")]
pub use number::*;

mod scalar;
pub use scalar::*;

mod scalar_type;
pub use scalar_type::*;

#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
