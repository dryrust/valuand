// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<isize> for Decimal {
    fn from(input: isize) -> Self {
        Self(input.into())
    }
}
