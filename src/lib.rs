// This is free and unencumbered software released into the public domain.

//! This crate implements a universal scalar value (aka variant) type for Rust.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
