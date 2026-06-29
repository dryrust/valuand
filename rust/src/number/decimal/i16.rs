// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i16> for Decimal {
    fn from(input: i16) -> Self {
        Self(input.into())
    }
}
