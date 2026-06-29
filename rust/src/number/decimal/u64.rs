// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<u64> for Decimal {
    fn from(input: u64) -> Self {
        Self(input.into())
    }
}
