// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<u16> for Decimal {
    fn from(input: u16) -> Self {
        Self(input.into())
    }
}
