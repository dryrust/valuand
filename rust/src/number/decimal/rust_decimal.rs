// This is free and unencumbered software released into the public domain.

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for Decimal {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self(input)
    }
}

#[cfg(feature = "rust_decimal")]
impl From<Decimal> for rust_decimal::Decimal {
    fn from(input: Decimal) -> Self {
        input.0
    }
}
