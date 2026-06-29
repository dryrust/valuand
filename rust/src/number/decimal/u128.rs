// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<u128> for Decimal {
    fn from(input: u128) -> Self {
        Self(input.into())
    }
}
