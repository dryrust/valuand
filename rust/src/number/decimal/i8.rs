// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i8> for Decimal {
    fn from(input: i8) -> Self {
        Self(input.into())
    }
}
