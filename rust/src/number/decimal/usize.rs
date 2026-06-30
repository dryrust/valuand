// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<usize> for Decimal {
    fn from(input: usize) -> Self {
        Self(input.into())
    }
}
