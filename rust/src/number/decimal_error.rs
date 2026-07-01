// This is free and unencumbered software released into the public domain.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DecimalError;

impl core::error::Error for DecimalError {}

impl core::fmt::Display for DecimalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DecimalError")
    }
}

impl From<rust_decimal::Error> for DecimalError {
    fn from(_: rust_decimal::Error) -> Self {
        DecimalError
    }
}
